# Project plan: "Ritm" — AI headway, crowd and reserve-fleet management for Baku buses

Hackathon: "İnsanyönümlü şəhərlər" (Youth Foundation + National AI Centre). Registration deadline: 08.11.2026. Stage I: idea/project pitch (150 participants). Stage II: Startup Demo Day (10 finalists). Working name "Ritm" (rhythm) — rename freely.

---

## 1. What we are building (one paragraph)

A software layer that sits on top of AYNA's existing bus GPS/IoT data and does three things: (1) **Headway Guardian** — detects buses starting to bunch and tells dispatchers/drivers exactly how long to hold at the next control stop so intervals stay even; (2) **Crowd Forecast** — predicts, 15–60 minutes ahead, which stops and corridors will overflow, using tap-in history, metro exits, the location of schools, universities, shopping centres and offices around each stop, time/weather/events — and uses the buses' own GPS delays as a live traffic sensor (where buses slow down is where traffic is); (3) **Reserve Dispatch** — manages a standby pool of ~20 buses/minibuses parked at 3–4 hubs and recommends when and where to insert one to fill a predicted gap or crowd. All three share one data pipeline, one map, one set of KPIs. The AI is honest and defensible: gradient-boosted prediction models + a proven headway-control rule with safety guardrails + an optimisation solver for reserve assignment. No reinforcement learning as the live controller.

**Who pays:** AYNA. From Sept 2026 gross-cost contracts (routes 130, 205, 211, 500, 503) make interval adherence a financially penalised KPI; AYNA announced a Data Warehouse (Jul 2026) and tendered a ~3M AZN mini-ITS with an AI agent (Oct 2026). We are building the module they are shopping for.

---

## 2. The three modules in detail

### 2.1 Headway Guardian (core — must work on Demo Day)
- Input: vehicle positions every 10–30 s per route.
- Geofence each stop → stop arrival/departure events → live headways.
- KPIs per route: headway coefficient of variation (CV), Excess Wait Time (EWT = actual − scheduled average wait), % bunched (headway < 25% of planned), % big gaps (> 150%).
- Controller: *two-way-looking even-headway holding* (look at the bus ahead and the bus behind; hold = ½·(gap behind − gap ahead), clipped to 0–90 s, never hold a bus that is already late beyond threshold).
- **What "hold" means — and does not mean.** The bus stops at every stop as normal and lets everyone board and alight, however long that takes (elderly passengers, wheelchairs, strollers). Only *after* boarding is finished, the tablet may show "earliest departure in 40 s". It is a *minimum departure time*, never a command to leave, never a command to skip a stop, and never a command to shorten boarding. If boarding already took longer than the suggested hold, the hold is simply zero. **Stop-skipping and short-turning are excluded from this project — they are against the rules and not needed.** This is the rule that produced −26% excess wait at WMATA (Washington) and −9…−20% wait at Chicago CTA even with trips cancelled and only a third of drivers complying.
- Terminal dispatch regularity: even departures from terminals (highest-leverage point; downstream irregularity ≈ 0.69× dispatch irregularity in Bogotá data).
- Output: one-line instruction on a driver tablet/phone ("Hold 40 s at stop 12") and a dispatcher console.

### 2.2 Crowd Forecast
- Target: predicted boardings per stop per 15-min slot, and predicted on-board load per trip.
- Features: historical BakuBus daily boardings (opendata.az), metro station entries (opendata.az / metro.gov.az), hour, weekday, holiday, weather, known events (stadium, concerts, exam days), school/university calendar.
- **Place features (what is around each stop):** count and type of points of interest within 300–500 m — schools, universities, shopping centres, markets, hospitals, offices, metro entrances — from OpenStreetMap / GoMap. A stop next to a university behaves differently at 08:30 and 17:30 than a stop next to a shopping mall; the model learns these profiles per stop and per hour.
- **Traffic from the buses themselves:** every bus is a moving traffic sensor. Link travel time vs the free-flow time for that link = a live congestion index per street, updated every minute, with no cameras or external APIs. Aggregate it into a congestion map; feed it to the arrival-time predictor and the crowd model (a jammed corridor → later buses → more people accumulating at downstream stops). Yandex/GoMap traffic is an optional cross-check only.
- Model: LightGBM / CatBoost regressor (fast to train, explainable with SHAP). Baseline to beat: "same slot last week".
- No cameras anywhere in this project (not in the demo, not in the roadmap). Demand is estimated from tap-ins, metro exits, place features and bus delays.

### 2.3 Reserve Dispatch (the "20 emergency buses")
- Standby pool: ~20 vehicles (mix of 12 m buses and 8 m minibuses) at 3–4 hubs (e.g., 28 May/Nizami area, Koroğlu, Əhmədli, Memar Əcəmi — to be validated against route map).
- Trigger: predicted gap > 2× planned headway on a high-frequency route OR predicted load > 110% capacity at a stop cluster within 30 min OR metro disruption flag.
- Decision: assignment problem — which reserve vehicle, from which hub, inserts where, on which route, for how many trips. Solve with Google OR-Tools (min-cost assignment) with constraints: deadhead travel time from hub, driver shift limits, vehicle type vs route (minibus cannot run articulated-only routes), max simultaneous deployments.
- Output: dispatcher card "Send reserve M-07 from Koroğlu hub to route 49, insert after bus 2344 at stop Gənclik, ETA 11 min. Expected gain: wait −6 min for ~420 passengers." Dispatcher approves or declines in one tap; every decision logged for learning.
- Honest framing: reserve insertion is a *supplement* to holding control, not a replacement for buying buses or enforcing bus lanes.

---

## 3. Team and roles (4–5 people)

| Role | Owns | Skills |
|---|---|---|
| Data/backend lead | ingestion, stop-event pipeline, KPIs, API | Python, PostgreSQL/PostGIS or DuckDB, FastAPI |
| ML lead | crowd forecast, arrival-time prediction, SHAP explanations | pandas, LightGBM, scikit-learn |
| Simulation/optimisation | line simulator, holding controller, reserve assignment | SimPy, OR-Tools, NumPy |
| Frontend | dispatcher console, driver tablet view, passenger view | React/Vite, MapLibre GL or deck.gl, Tailwind |
| Pitch/product (can be shared) | problem story, AYNA contact, slides, Azerbaijani copy, video | — |

If only 3 people: merge Data+ML, and Simulation+Frontend; everyone pitches.

---

## 4. Timeline

Dates for Stage I and Demo Day are not yet published; plan assumes ~2 weeks to Stage I and ~4–6 weeks to Demo Day. Compress or stretch accordingly.

### Week 0 (1–8 Nov): Register and lock scope
- [ ] Register all team members before **08.11.2026** (forms.gle/Ecakm2MWMUgXSeZQ6).
- [ ] Write the 1-page project description (AZ + EN) — problem, solution, AI, data, impact, team.
- [ ] Create GitHub repo, Trello/Notion board, shared drive.
- [ ] Start **data collection immediately** (it needs calendar time): a script polling AYNA MaaS app vehicle positions for 3–5 target routes every 20 s, 24/7. Target routes: 49 (documented bunching), 52 or 96 (long waits), and one gross-cost route (130/205/211/500/503). Store raw JSON with timestamps.
- [ ] Download AYNA route/stop geometry (public map API; existing scraped snapshots + GTFS on GitHub/Zenodo as fallback), BakuBus boardings and metro entries from opendata.az.
- [ ] Email/LinkedIn AYNA innovation or digital department and the National AI Centre mentor list: ask for (a) a letter of interest, (b) a sample of historical AVL data for one route. Not required for the demo, but one reply is worth a slide.

### Week 1 (9–15 Nov): Data pipeline + evidence
- [ ] Clean positions → map-match to route shape → geofence stops → stop events table.
- [ ] Compute headway CV, EWT, % bunched per route per hour. **Produce the first real Baku time–space diagram showing bunching.** This single chart is the heart of the Stage I pitch.
- [ ] Quantify: "On route 49 between 17:00–19:00, average wait was X min vs Y planned; Z% of buses arrived < 2 min after the previous one."
- [ ] Baseline forecast model (last-week-same-slot) and first LightGBM crowd model on opendata boardings.
- [ ] Stage I pitch deck v1 (8 slides) using the real chart.

### Week 2 (16–22 Nov): Simulator + controller
- [ ] Event-based single-line simulator (SimPy): buses, stops, Poisson passenger arrivals, boarding time per passenger, link travel times drawn from the *observed* Baku distributions (so the simulated CV matches reality without control).
- [ ] Implement: no control / schedule-based / terminal even dispatch / two-way holding. Reproduce the known result: holding cuts CV and EWT sharply, and re-equalises after a bus is removed.
- [ ] Add disturbances: remove 1 bus (fleet shortage), inject a 10-min traffic jam on one link, surge of 200 passengers at one stop (metro exit).
- [ ] Unit tests for the holding rule guardrails (max hold, no-hold stops, lateness threshold).

### Week 3 (23–29 Nov): Reserve Dispatch + ML
- [ ] Reserve pool model: hubs, vehicles, deadhead times from GoMap/OSRM routing.
- [ ] OR-Tools assignment: trigger → candidate insertions → cost (deadhead + driver time − passenger-minutes saved) → recommendation.
- [ ] Crowd forecast v2 with weather/events/metro features; report MAE vs baseline; SHAP plot for explainability.
- [ ] Arrival-time predictor (LightGBM on link times, hour, weekday, traffic index) — show error vs schedule-based ETA (AYNA's app currently shows schedule-based times).
- [ ] Congestion map from bus GPS: per-link travel-time index vs free-flow, rendered on the map; validate against a few Yandex screenshots.
- [ ] POI feature table per stop from OpenStreetMap (schools, universities, malls, hospitals, offices, metro).

### Week 4 (30 Nov–6 Dec): Product UI + integration
- [ ] Dispatcher console (web): live map (MapLibre + route shapes), per-route KPI tiles, bunching alerts, reserve recommendation cards with approve/decline, scenario switch (replay real day vs simulate with control).
- [ ] Driver tablet view (phone-sized): big "HOLD 40 s" countdown, next stop, gap ahead/behind.
- [ ] Passenger view: next two buses, predicted crowding (green/amber/red) — mirrors Istanbul İETT hackathon winners.
- [ ] Optional LLM layer: natural-language questions over KPIs ("Which routes bunched most this week and why?") using Claude API with tool calls to the KPI database. Keep it as a query layer, not the controller.
- [ ] End-to-end rehearsal with a fixed 6-minute demo script (§5).

### Week 5–6 (if Demo Day is later): Hardening and pilot design
- [ ] Replay 2 full weeks of real route-49 data through the controller offline; report counterfactual gain with confidence interval.
- [ ] Pilot proposal document for AYNA: 1 route, 8 weeks, success metric EWT −15%, no hardware needed (uses existing GPS + a tablet per bus).
- [ ] Record a 90-second video of the demo as a backup against Wi-Fi failure.

---

## 5. Demo Day script (6 minutes)

1. **(0:00) The problem, with our own data.** Real time–space diagram of route 49 on a weekday evening: three buses arrive within 2 minutes after a 25-minute gap. "This is not a schedule problem. It is a control problem."
2. **(1:00) Headway Guardian live.** Replay the same evening with the controller on: gaps even out; KPI tiles show CV 0.7 → 0.3, EWT −X min. Show the driver tablet instruction appearing.
3. **(2:30) Stress test.** Click "remove a bus" (fleet shortage) and "inject traffic jam": the line re-equalises itself within 2 cycles.
4. **(3:30) Crowd Forecast + traffic from buses.** Congestion map built only from bus GPS lights up on Nizami/Babek corridor; forecast heat map shows the university stops turning red at 17:30; explain the place features (schools, universities, malls) behind the prediction.
5. **(4:15) Reserve Dispatch.** The system proposes sending reserve M-07 from Koroğlu hub, with ETA and passengers helped; dispatcher approves in one tap; map shows the insertion closing the gap.
6. **(5:15) Business and path.** AYNA already pays penalties/incentives on interval KPIs under gross-cost contracts; AYNA's own 3M AZN ITS tender includes an AI agent; our pilot: 1 route, 8 weeks, zero hardware. Ask: a data-sharing agreement and a pilot route.

---

## 6. Data plan

| Need | Source now (no AYNA permission) | Source with AYNA | Notes |
|---|---|---|---|
| Routes, stops, shapes | AYNA public map API; scraped GTFS exports on GitHub/Zenodo | Official GTFS | Unlicensed — say "publicly accessible, pilot would use official feed" |
| Live vehicle positions | Poll AYNA MaaS app endpoints (Dec 2025 app shows live positions) — confirmed easy to obtain | AVL feed (GTFS-RT or direct) | Start polling in Week 0; store everything; this is the primary data source |
| Boardings/demand | BakuBus daily boardings, metro entries (opendata.az) | BakıKart tap-ins per stop per minute | Tap data is the big unlock for crowd prediction |
| Traffic | Derived from bus GPS link travel times (own congestion index) | same, plus NİİM feeds if offered | Yandex/GoMap only as a cross-check |
| Places around stops | OpenStreetMap / GoMap POIs: schools, universities, malls, hospitals, offices, metro | AYNA stop metadata | Static table, built once |
| Reserve fleet | Assumed 20 vehicles, 4 hubs | Real depot locations, shift rules | State assumptions clearly |

Fallback if live polling fails: synthetic AVL generated by the simulator calibrated to published Baku bus speeds (18→22 km/h) and headways — label it clearly as synthetic.

---

## 7. Architecture and stack

```
[AYNA MaaS positions poll] [opendata.az boardings/metro] [OSM/GoMap places]  [bus-GPS congestion index]
            │                │                │                      │
            └────────► Ingestion (Python, FastAPI jobs) ◄────────────┘
                                   │
                    PostgreSQL + PostGIS (or DuckDB for hackathon)
                                   │
        ┌──────────────────────────┼──────────────────────────┐
  Stop events / KPIs        Forecast models              Line simulator
  (headway, EWT, bunching)  (LightGBM: demand, ETA)      (SimPy, calibrated)
        │                          │                          │
        └──────► Decision engine: holding rule + guardrails, reserve assignment (OR-Tools)
                                   │
                         REST/WebSocket API
                                   │
     Dispatcher console (React + MapLibre)   Driver tablet (PWA)   Passenger view (PWA)
                         Optional: LLM query layer over KPIs (Claude API, tool calls)
```

- Python 3.11, FastAPI, pandas, LightGBM, SimPy, OR-Tools, OSMnx (places around stops).
- React + Vite + TypeScript, MapLibre GL JS, Recharts for KPI charts.
- Docker Compose for one-command start; GitHub Actions for tests.

---

## 8. Metrics to show (and targets)

| Metric | Definition | Baku today (measure it) | Target in sim |
|---|---|---|---|
| Headway CV | std/mean of headways | likely 0.6–0.9 peak | ≤ 0.3 |
| EWT | actual avg wait − scheduled avg wait | measure | −20% or more |
| % bunched | headway < 25% of planned | measure | ≤ 5% |
| % big gaps | headway > 150% of planned | measure | ≤ 5% |
| Forecast MAE | boardings/15 min | baseline last-week | −30% vs baseline |
| ETA error | predicted vs actual arrival | schedule-based | halve it |
| Reserve response | trigger → insertion | n/a | ≤ 15 min, passenger-minutes saved per deployment |

---

## 9. Numbers to cite (all in the research report with sources)

- Baku: ~494M bus passengers/yr, ~2,300–2,530 buses/day, 201 routes, 39 carriers; ~1,000 buses need replacement; 114–117 km bus lanes; complaints on routes 49, 52, 96.
- WMATA headway control: −26% excess wait (~US$1M/yr). CTA 2022–23: −5.8% to −19.9% wait with 15–25% trips cancelled and 35–57% driver compliance. Stockholm: −10% wait, −25% irregularity. Singapore BSRF incentives: −15–25% EWT.
- Terminal dispatch: downstream CV ≈ 0.69× dispatch CV (Bogotá).
- ML ETA: GNN+LSTM halves error vs schedule baseline (MAE 13 s vs 26 s).
- Boarding: all-door boarding −38% dwell (SF); off-board payment −63% bunching (NYC SBS) — recommend as policy complement.

## 10. Do NOT claim
- Reinforcement learning as the controller (it beats a tuned rule by ~1.5–3% and has one real deployment).
- Shenzhen "−33% waiting" or Dubai "+50% punctuality" (undefined metrics).
- Vendor on-time-performance figures as headway results.
- Access to AYNA GPS/fare data or NİİM cameras that we don't have.
- That software replaces bus lanes or new buses.
- Guaranteed driver compliance — show results *at* realistic compliance (35–60%).
- Anything involving cameras, stop-skipping, short-turning or cutting boarding time short.

## 11. Risks and mitigations

| Risk | Mitigation |
|---|---|
| AYNA app endpoints change or block polling | Cache from day one; GTFS snapshot fallback; synthetic mode labelled |
| Too little real data before Stage I | Even 3 days of one route is enough for one honest chart |
| Scope creep (3 modules) | Headway Guardian is the must-have; Forecast and Reserve are demo-quality; cut Reserve UI before cutting Guardian |
| Judges ask "why not just more buses?" | Show the remove-a-bus stress test: control is what makes the fleet you have usable |
| Demo Wi-Fi fails | Everything runs locally from Docker; pre-recorded video backup |
| Judges ask how we see traffic without cameras | Show the bus-GPS congestion map: 2,500 buses are already 2,500 traffic sensors |
| Judges ask about drivers being forced to leave or skip stops | Show the tablet: only an *earliest departure time* after boarding is complete; all stops always served |

## 12. First five tasks (do today)
1. Register for the hackathon.
2. Start the position-polling script for routes 49, 52/96 and one gross-cost route; let it run.
3. Pull route/stop geometry, opendata.az boardings, and OpenStreetMap places (schools, universities, malls) around each stop.
4. Create repo with the folder layout: `ingest/`, `kpi/`, `sim/`, `ml/`, `dispatch/`, `web/`, `docs/`.
5. Draft the 1-page project description in Azerbaijani.
