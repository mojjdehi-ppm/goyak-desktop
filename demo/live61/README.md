# Ritm live demo: Baku bus line 61

`ritm-61.html` is a single self-contained page: real route 61 (Azadlıq pr. – 9-cu mkr) on a
street map of central Baku, with simulated buses and passengers and the Ritm controls.

## What is real and what is simulated

| Real | Simulated |
|---|---|
| Route 61 stops (53, both directions) and shape, from AYNA's public map | Number of buses, interval (AYNA publishes no timetable) |
| Streets, street names, metro lines and stations (OpenStreetMap) | Bus speeds, traffic, passenger arrivals |
| Place types around each stop: schools, universities, clinics, offices, metro (OpenStreetMap, 400 m) | Reserve pool (3 + 3 vehicles at the terminals) |

## Rebuild

```
git clone --depth 1 https://github.com/ismatsamadov/route_efficiency
python3 -I build_data.py route_efficiency route61.json
python3 -I build_page.py route61.json        # writes ritm-61.html from template.html
```

## Sources and licences

- Route 61 stops and shape: AYNA public map snapshot in
  [route_efficiency](https://github.com/ismatsamadov/route_efficiency) (MIT).
- Streets, metro, places: © OpenStreetMap contributors, ODbL.
