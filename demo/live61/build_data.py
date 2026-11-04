"""Build the data bundle for the Ritm route-61 live demo.

Inputs (from github.com/ismatsamadov/route_efficiency, MIT; map data (c) OpenStreetMap contributors, ODbL):
  output/redesign/gtfs_today.zip   route 61 stops and shapes, modelled from AYNA's public map
  data/external/osm/*.json(.gz)    streets, POIs, metro stations
  data/metro/routes.geojson        metro lines

Usage: python3 -I build_data.py <path-to-route_efficiency-clone> <out.json>
"""
import csv, gzip, io, json, math, sys, zipfile

SRC, OUT = sys.argv[1], sys.argv[2]
ROUTE_ID = '47'  # route_short_name 61
LAT0, LON0 = 40.398, 49.828
KX = math.cos(math.radians(LAT0)) * 111320
KY = 110540
PAD = 500  # metres of map around the route


def proj(lat, lon):
    return ((lon - LON0) * KX, -(lat - LAT0) * KY)


def dp(pts, tol):
    """Douglas-Peucker simplification."""
    if len(pts) < 3:
        return pts
    (x1, y1), (x2, y2) = pts[0], pts[-1]
    dx, dy = x2 - x1, y2 - y1
    L = math.hypot(dx, dy) or 1e-9
    imax, dmax = 0, -1
    for i in range(1, len(pts) - 1):
        x, y = pts[i]
        d = abs(dy * x - dx * y + x2 * y1 - y2 * x1) / L
        if d > dmax:
            imax, dmax = i, d
    if dmax <= tol:
        return [pts[0], pts[-1]]
    return dp(pts[:imax + 1], tol)[:-1] + dp(pts[imax:], tol)


# ---------- GTFS ----------
z = zipfile.ZipFile(f'{SRC}/output/redesign/gtfs_today.zip')
rd = lambda n: list(csv.DictReader(io.TextIOWrapper(z.open(n), encoding='utf-8')))
stops = {r['stop_id']: r for r in rd('stops.txt')}
trips = [r for r in rd('trips.txt') if r['route_id'] == ROUTE_ID]
st_all = rd('stop_times.txt')
shapes_all = rd('shapes.txt')
route = next(r for r in rd('routes.txt') if r['route_id'] == ROUTE_ID)
freq = {r['trip_id']: int(r['headway_secs']) for r in rd('frequencies.txt')}

dirs = []
for t in sorted(trips, key=lambda r: r['direction_id']):
    st = sorted((r for r in st_all if r['trip_id'] == t['trip_id']), key=lambda r: int(r['stop_sequence']))
    sh = sorted((r for r in shapes_all if r['shape_id'] == t['shape_id']), key=lambda r: int(r['shape_pt_sequence']))
    line = [proj(float(r['shape_pt_lat']), float(r['shape_pt_lon'])) for r in sh]
    dirs.append({'trip': t, 'stops': st, 'line': line})

# Build one loop: direction 0 (Mahnı Teatrı -> 9 Poliklinika) then direction 1 back.
loop, loop_stops = [], []


def cum(line):
    out, s = [0.0], 0.0
    for a, b in zip(line, line[1:]):
        s += math.hypot(b[0] - a[0], b[1] - a[1]); out.append(s)
    return out


def project_onto(line, c, p, start_d):
    """Distance along line of the closest point to p, searching forward from start_d."""
    best, bd = start_d, 1e18
    for i in range(len(line) - 1):
        if c[i + 1] < start_d - 1:
            continue
        (x1, y1), (x2, y2) = line[i], line[i + 1]
        dx, dy = x2 - x1, y2 - y1
        L2 = dx * dx + dy * dy or 1e-9
        u = max(0, min(1, ((p[0] - x1) * dx + (p[1] - y1) * dy) / L2))
        qx, qy = x1 + u * dx, y1 + u * dy
        d = (qx - p[0]) ** 2 + (qy - p[1]) ** 2
        dist = c[i] + u * math.sqrt(L2)
        if d < bd and dist >= start_d - 1:
            bd, best = d, dist
    return best


offset = 0.0
for k, D in enumerate(dirs):
    line = D['line']
    c = cum(line)
    pos = 0.0
    seq = D['stops'] if k == 0 else D['stops'][1:-1]  # drop duplicated terminals in the return trip
    if k == 1:
        pass
    for r in seq:
        s = stops[r['stop_id']]
        p = proj(float(s['stop_lat']), float(s['stop_lon']))
        pos = project_onto(line, c, p, pos)
        loop_stops.append({'name': s['stop_name'].strip(), 'lat': float(s['stop_lat']), 'lon': float(s['stop_lon']),
                           'd': round(offset + pos, 1), 'dir': k, 'xy': [round(p[0]), round(p[1])]})
    pts = line if k == 0 else line
    loop.extend([[round(x, 1), round(y, 1), k] for x, y in pts])
    offset += c[-1]
LOOP_LEN = offset
loop_stops[0]['terminal'] = True
term2 = len(dirs[0]['stops']) - 1
loop_stops[term2]['terminal'] = True
for i, s in enumerate(loop_stops):
    s['i'] = i

# ---------- POIs near stops ----------
def load(name):
    return json.load(open(f'{SRC}/data/external/osm/{name}.json'))['elements']


def classify(tags):
    a, sh, hc, of = tags.get('amenity'), tags.get('shop'), tags.get('healthcare'), tags.get('office')
    if a in ('university', 'college'):
        return 'university'
    if a == 'school':
        return 'school'
    if a in ('hospital', 'clinic') or hc in ('hospital', 'clinic'):
        return 'health'
    if sh in ('mall', 'department_store') or a == 'marketplace':
        return 'mall'
    if of or a in ('townhall', 'courthouse'):
        return 'office'
    return None


pois = []
for f in ['osm_education', 'osm_health', 'osm_shopping', 'osm_offices']:
    for e in load(f):
        if 'lat' not in e:
            continue
        t = classify(e.get('tags', {}))
        if t:
            pois.append((t, proj(e['lat'], e['lon']), e.get('tags', {}).get('name', '')))
metro_st = [e for e in load('osm_metro_stations') if e.get('tags', {}).get('station') == 'subway' or e.get('tags', {}).get('railway') == 'station']
metros = []
seen = set()
for e in metro_st:
    n = e['tags'].get('name', '')
    if n in seen or 'lat' not in e:
        continue
    seen.add(n); metros.append({'name': n, 'xy': proj(e['lat'], e['lon'])})

WEIGHT = {'university': 0.45, 'school': 0.12, 'health': 0.18, 'mall': 0.25, 'office': 0.05}
for s in loop_stops:
    x, y = s['xy']
    cnt = {k: 0 for k in WEIGHT}
    names = {k: [] for k in WEIGHT}
    for t, (px, py), nm in pois:
        if math.hypot(px - x, py - y) <= 400:
            cnt[t] += 1
            if nm and len(names[t]) < 2:
                names[t].append(nm)
    near_metro = [m['name'] for m in metros if math.hypot(m['xy'][0] - x, m['xy'][1] - y) <= 300]
    w = 0.5 + sum(min(cnt[k], 8) * WEIGHT[k] for k in WEIGHT) + (1.2 if near_metro else 0)
    nm = s['name'].lower()
    if near_metro or 'm/st' in nm:
        place = 'metro'
    elif any(k in nm for k in ('poliklinika', 'xəstəxana', 'anatomiya')):
        place = 'health'
    elif any(k in nm for k in ('universitet', 'akademiya', 'konservatoriya')) or cnt['university'] >= 2:
        place = 'university'
    elif 'məktəb' in nm or 'bağça' in nm or cnt['school'] >= 4:
        place = 'school'
    elif any(k in nm for k in ('poliklinika', 'xəstəxana', 'anatomiya')) or cnt['health'] >= 9:
        place = 'health'
    elif cnt['mall'] >= 2:
        place = 'mall'
    elif cnt['office'] >= 12:
        place = 'office'
    else:
        place = 'residential'
    s.update({'w': w, 'place': place, 'poi': cnt, 'metro': near_metro[:1], 'poiNames': {k: v for k, v in names.items() if v}})
mean_w = sum(s['w'] for s in loop_stops) / len(loop_stops)
for s in loop_stops:
    s['w'] = round(s['w'] / mean_w, 3)

# ---------- base map ----------
xs = [p[0] for p in loop] + [s['xy'][0] for s in loop_stops]
ys = [p[1] for p in loop] + [s['xy'][1] for s in loop_stops]
minx, maxx, miny, maxy = min(xs) - PAD, max(xs) + PAD, min(ys) - PAD, max(ys) + PAD
# make the frame a bit wider than tall
w, h = maxx - minx, maxy - miny
if w < h * 1.05:
    cx = (minx + maxx) / 2; w = h * 1.05; minx, maxx = cx - w / 2, cx + w / 2
inside = lambda x, y: minx - 300 <= x <= maxx + 300 and miny - 300 <= y <= maxy + 300

net = json.load(gzip.open(f'{SRC}/data/external/osm/osm_walk_network.json.gz'))['elements']
nodes = {e['id']: proj(e['lat'], e['lon']) for e in net if e['type'] == 'node'}
CLASS = {'motorway': 1, 'trunk': 1, 'primary': 1, 'motorway_link': 2, 'trunk_link': 2, 'primary_link': 2, 'secondary': 2,
         'secondary_link': 3, 'tertiary': 3, 'tertiary_link': 3, 'unclassified': 4, 'residential': 4, 'living_street': 4,
         'busway': 3, 'pedestrian': 5}
TOL = {1: 2.5, 2: 2.5, 3: 3, 4: 4, 5: 4}
streets = {1: [], 2: [], 3: [], 4: [], 5: []}
named = {}
for e in net:
    if e['type'] != 'way':
        continue
    hw = e['tags'].get('highway')
    k = CLASS.get(hw)
    if not k:
        continue
    pts = [nodes[n] for n in e['nodes'] if n in nodes]
    if len(pts) < 2 or not any(inside(x, y) for x, y in pts):
        continue
    pts = dp(pts, TOL[k])
    streets[k].append([v for x, y in pts for v in (round(x), round(y))])
    nm = e['tags'].get('name')
    if nm and k <= 2:
        ins = [p for p in pts if minx + 400 < p[0] < maxx - 400 and miny + 300 < p[1] < maxy - 300]
        L = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(ins, ins[1:]))
        prev = named.get(nm, (0, 0, None))
        if len(ins) >= 2 and L > prev[1]:
            mid = ins[len(ins) // 2]
            a, b = ins[max(0, len(ins) // 2 - 1)], ins[min(len(ins) - 1, len(ins) // 2 + 1)]
            if a == b: a, b = ins[0], ins[-1]
            ang = math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))
            if ang > 90: ang -= 180
            if ang < -90: ang += 180
            named[nm] = (prev[0] + L, L, (round(mid[0]), round(mid[1]), round(ang)))
        elif L:
            named[nm] = (prev[0] + L, prev[1], prev[2])
labels = [{'n': n, 'x': v[2][0], 'y': v[2][1], 'a': v[2][2]} for n, v in sorted(named.items(), key=lambda kv: -kv[1][0])
          if v[2] and v[0] > 800 and v[1] > 250][:16]

mg = json.load(open(f'{SRC}/data/metro/routes.geojson'))
COL = {'red': 'red', 'green': 'green', 'purple': 'purple', 'light green': 'green'}
metro_lines, seen_lines = [], set()
for f in mg['features']:
    line = f['properties'].get('line')
    pts = [proj(la, lo) for lo, la in f['geometry']['coordinates']]
    pts = dp(pts, 4)
    if not any(inside(x, y) for x, y in pts):
        continue
    key = (line, round(pts[0][0]), round(pts[-1][0]))
    if key in seen_lines:
        continue
    seen_lines.add(key)
    metro_lines.append({'line': line, 'c': f['properties'].get('colour', ''), 'p': [v for x, y in pts for v in (round(x), round(y))]})
metro_pts = [{'n': m['name'], 'x': round(m['xy'][0]), 'y': round(m['xy'][1])} for m in metros if inside(*m['xy'])]

out = {
    'route': {'short': route['route_short_name'], 'long': route['route_long_name'], 'modelHeadway': freq.get(dirs[0]['trip']['trip_id'])},
    'dirNames': [d['trip']['trip_headsign'] for d in dirs],
    'loop': loop, 'loopLen': round(LOOP_LEN, 1), 'dir0Len': round(cum(dirs[0]['line'])[-1], 1),
    'stops': [{k: s[k] for k in ('i', 'name', 'd', 'dir', 'xy', 'w', 'place', 'metro', 'poi') if k in s} | ({'terminal': True} if s.get('terminal') else {}) for s in loop_stops],
    'bbox': [round(minx), round(miny), round(maxx - minx), round(maxy - miny)],
    'streets': {str(k): v for k, v in streets.items()},
    'labels': labels, 'metroLines': metro_lines, 'metroStations': metro_pts,
}
json.dump(out, open(OUT, 'w', encoding='utf-8'), ensure_ascii=False, separators=(',', ':'))
print('stops', len(loop_stops), 'loop km', round(LOOP_LEN / 1000, 2), 'streets', {k: len(v) for k, v in streets.items()},
      'labels', len(labels), 'metro lines', len(metro_lines), 'stations', len(metro_pts))
import collections
print('places', collections.Counter(s['place'] for s in loop_stops))
