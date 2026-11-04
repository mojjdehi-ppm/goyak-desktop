# Baku's Public Bus System under AYNA (Azərbaycan Yerüstü Nəqliyyat Agentliyi) — Network, Technology, Reliability, Open Data, Reforms (2023–2026)

Research date: 1 November 2026. Facts older than 2023 are marked "(older)". Azerbaijani names are given in original spelling with English gloss.

Institutional note: AYNA (Azərbaycan Yerüstü Nəqliyyat Agentliyi / Azerbaijan Land Transport Agency) sits under the Rəqəmsal İnkişaf və Nəqliyyat Nazirliyi (RİNN / Ministry of Digital Development and Transport) and is the successor of BNA (Bakı Nəqliyyat Agentliyi / Baku Transport Agency). BakuBus MMC is the single state-owned operator; all other routes are run by private carriers ("daşıyıcı") under AYNA contracts.

---

## Key Question 1 — Network size: routes, buses, passengers, modal share, operators, and the 2024–2026 route reform

### Takeaway
Baku's bus network is ~200 routes run by ~39 carriers with a record ~2,300–2,530 buses in daily operation (up ~42% since 2022), carrying roughly 494 million passengers a year (~1.3–1.4 million/day) versus ~627,000/day on the metro. A "new network" (yeni şəbəkə) is being rolled out piecemeal (8 routes live by Jan 2026, plus 8 M-indexed express routes created in Aug 2026 during the metro line-separation closure), and the contracting model is shifting from per-passenger to gross-cost ("brutto müqavilə") contracts starting Sept 2026.

### Cited Findings
**Size and ridership**
- AYNA's public map snapshot contains 201 routes operated by 39 carriers serving 3,456 stops (research snapshot of AYNA map API, 2026) — [ismatsamadov/route_efficiency (GitHub)](https://github.com/ismatsamadov/route_efficiency); the accompanying preprint says "Baku's bus network of about 200 routes is being redesigned" and only 29 of 201 routes run within 1.4× the straight line between their ends; 66% of 3.04 million residents live within 400 m of a stop — [Samadov, engrXiv preprint, 27 Oct 2026](https://engrxiv.org/preprint/view/8632)
- Daily active fleet (September of each year, per AYNA): 1,780 (2022); 2,011 (2023); 2,180 (2024); 2,340 (2025); 2,530 in Oct 2026, "a record" — growth of ~750 buses (~42%) in four years. 308 new buses arrived in 2026 by October; 29 routes re-equipped with newer buses in 2026 — [1news.az, 2 Oct 2026 (AYNA statements)](https://1news.az/az/news/20261002150642203-Bakida-sinaga-chevrilen-avtobus-seferi-AYNA-sixliq-kondisioner-ve-davranish-qaydalari-ile-bagli-neleri-deyir)
- A slightly different official series from the State Program roundtable: 1,720 buses daily in 2022, 1,950 in 2023, 2,160 in 2024, "around 2,300" in Jan 2026 (+32%); bus passenger transport in Baku and surroundings up 67.3 million over four years; 395 new electric/CNG buses put into service in Baku, Sumgayit and Absheron in 2025; eco-friendly buses = 55% of fleet; 98 extra buses added on Sumgayit/Saray routes — [Mincom (RİNN) roundtable, 28 Jan 2026](https://mincom.gov.az/en/media-en/news/roundtable-on-the-2025-state-program-for-the-improvement-of-transport-infrastructure-in-baku-and-surrounding-areas)
- At WUF13 (May 2026) Tarlan Safarov, head of AYNA's Baku Regional Department: 2,300 buses operating daily (+30%), ~494 million passengers/year, 368 "green energy" buses, 114 km of bus lanes, contactless payment on 1,900 buses; main focus is "a complete restructuring of Baku's bus system" under the State Program; slogan "One City. One Network. One Journey." — [APA, 21 May 2026](https://en.apa.az/wuf13/planned-railway-metro-and-tram-projects-for-passenger-transportation-in-baku-announced-507647)
- Jan–Aug 2026: 335.6 million passenger trips on Baku intracity, suburban and inter-settlement buses, +5.9% YoY — [Report.az, 28 Sept 2026](https://report.az/en/infrastructure/baku-bus-passenger-trips-reach-335-6-million-in-jan-aug-2026-ayna-says)
- BakuBus (state operator) self-reported: average daily turnover >350,000 passengers, ~23% share of Baku bus passenger transport, >750 million cumulative trips since 2015 (undated on site) — [bakubus.az](https://www.bakubus.az/); a separate aggregator cites BakuBus at 1,337 buses, 83 routes, >557,000 daily passengers in 2025 and >81 million passengers in Jan–Sept 2024 — [search summary citing azcon.gov.az/Wikipedia](https://azcon.gov.az/en/portfolio/bakubus-4) (secondary; treat with caution — conflicts with the 23%/350k figure on bakubus.az)
- Baku Metro 2024: 229.6 million riders, average 627,279/day — [Wikipedia: Baku Metro](https://en.wikipedia.org/wiki/Baku_Metro); AIIB project doc: ~650,000/day, metro = 30% of public transport trips; Baku ≈ 4.8 million motorized trips/day, ~31% by public transport — [AIIB Baku Metro Expansion Phase I PD](https://www.aiib.org/en/projects/details/2026/_download/Azerbaijan/Republic-of-Azerbaijan-Baku-Metro-Expansion-Project-Phase-I.pdf)
- (older, 2018) World Bank policy note described a roughly even private/public split and 1.624 million daily bus passengers — [World Bank Baku Urban Mobility Policy Note, June 2018](https://thedocs.worldbank.org/en/doc/963521580136216965-0080022020/original/BakuUrbanMobilityPolicyNoteJune2018.pdf)

**Route reform / new network / express routes**
- "The new network has been launched, with eight routes now operational" (as of Jan 2026) — [Mincom roundtable, 28 Jan 2026](https://mincom.gov.az/en/media-en/news/roundtable-on-the-2025-state-program-for-the-improvement-of-transport-infrastructure-in-baku-and-surrounding-areas)
- AYNA chairman Anar Rzayev (Jan 2026): ~1,000 Baku buses need replacement; fleet to reach 2,800 by 2030; new buses are red, old ones white; ~400 new buses to Baku in 2025; from 2026 a dedicated "Public Transport" budget fund and gross-cost contracts with carriers — [operativmm.az, 28 Jan 2026](https://operativmm.az/en/post/baku-bus-fleet-to-be-modernized-by-2030/66473)
- Aug 2026: metro suspended between "28 May" and "Nizami" for the Green/Red line separation; AYNA launched six express routes M1–M6 (06:00–00:00), added buses on ~20 regular routes, and moved terminals of routes 32, 49, 120E, 125 away from the 28 May interchange — [AZERTAC](https://azertag.az/xeber/sabahdan_bakida_6_yeni_ekspres_marsrutu_fealiyyete_baslayir-4365513); [APA](https://en.apa.az/infrastructure/six-new-express-bus-routes-launched-in-baku-ahead-of-suspension-of-metro-services-video-520319); by Oct 2026 AYNA said 8 express routes created and 28 regular routes strengthened — [1news.az, 2 Oct 2026](https://1news.az/az/news/20261002150642203-Bakida-sinaga-chevrilen-avtobus-seferi-AYNA-sixliq-kondisioner-ve-davranish-qaydalari-ile-bagli-neleri-deyir)
- Baku Metro used AI tools on passenger entry/exit data (incl. transfers at 20 Yanvar, Gənclik) and shared results with AYNA to design alternative bus lines; ~1,600 passengers arrive per train at Nizami every 3 minutes (≈20 buses/3 min equivalent) — [Trend interview with Vusal Aslanov, 12 May 2026](https://www.trend.az/azerbaijan/society/4184934.html)
- Baku Metro CJSC is being reorganized by merging BakuBus and Baku Taxi Service LLCs into it, with data from all modes "processed in a single center" and a single payment/schedule system planned — [Trend, 12 May 2026](https://www.trend.az/azerbaijan/society/4184934.html)
- Gross-cost contracts: Cabinet of Ministers Decision No. 250 (31 July 2026) sets distance-based subsidy rules; first routes 130, 205, 211, 500, 503 (Sept 2026); carriers paid per km and service quality, not per passenger; required fleet, intervals and distance fixed per route; a "dedicated software tool" monitors quality; financial penalties for schedule breaches — [marja.az, 2 Sept 2026](https://marja.az/139391/dasiyicilara-mesafeye-gore-pul-odenilecek-bakida-avtobuslarla-bagli-yeni-sistem-ise-dusdu)
- Operator competition: AYNA put 37 regular routes to tender (Sept 2025) — [Report.az](https://report.az/infrastruktur/37-sayda-muntezem-avtobus-marsrutu-musabiqeye-cixarilir); 23 routes to tender (~Sept 2026) possibly changing operators on 22, 31, 67, 92, 172, 191 — [marja.az](https://marja.az/140098/bakida-6-avtobus-marsrutu-uzre-dasiyici-deyise-biler-22-31-67-92-172-ve-191-nomreli-avtobuslarla-bagli-yenilik); [APA](https://apa.az/infrastruktur/avtobus-marsrutlari-musabiqeye-cixarilir-910469)
- Multimodal hubs planned at Sumqayıt, Xırdalan, Sabunçu, Koroğlu (rail–metro–bus transfer under 2 min) — [APA, 21 May 2026](https://en.apa.az/wuf13/planned-railway-metro-and-tram-projects-for-passenger-transportation-in-baku-announced-507647)
- Express routes with "E" index exist (e.g. E1, E2 at 0.50 AZN; trial 149E Sahil–"Sahil" metro at 1.30 AZN with 6 Neoplan buses) — [ayna.gov.az](https://www.ayna.gov.az/az/news/e2-nomreli-yeni-ekspres-avtobus-marsrutu-istifadeye-verilir-37); [musavat.com](https://musavat.com/news/bakida-yeni-ekspres-marsrut-acilir_1269664.html) (dates not confirmed)

### Inferences
- Bus is by far the dominant public transport mode (roughly 2× metro ridership), so bus reliability dominates perceived PT quality.
- The two official fleet series (1,720 vs 1,780 for 2022, etc.) differ by ~50–60 buses, probably Baku-city-only vs Baku+Absheron scope; use ~2,300 (daily active, spring 2026) and ~2,500 (Oct 2026) with that caveat.
- The gross-cost contract rollout makes interval adherence a contractual, financially penalized KPI for the first time — creating demand for exactly the kind of monitoring/analytics a hackathon tool could provide.

### Gaps
- No official, current count of carriers (the 39 figure is from a scraped snapshot) or of routes per operator; nothing found specifically on Xaliq Faiqoğlu or "Bakı Avtobus Parkı" as operators.
- No published modal-share survey for 2024–2026 beyond the AIIB ~31% public-transport estimate.
- Full "yeni marşrut şəbəkəsi" design (feeder vs trunk hierarchy, target route count) not published; only "8 routes of the new network" are live.

---

## Key Question 2 — Fleet: size, age, electric buses, procurements, acknowledged bus and driver shortages

### Takeaway
Fleet renewal is fast (≈400 new buses in 2025, 308 by Oct 2026) and electrification is real (~368 e-buses by spring 2026, mostly BYD K9UD, now locally assembled in Sumqayıt), but AYNA itself says ~1,000 buses still need replacing, and AYNA deflects driver-shortage questions to carriers while BakuBus advertises 1,350 AZN gross for drivers.

### Cited Findings
- Fleet renewal need: ~1,000 buses need replacement; staged renewal 2025–2030; fleet to 2,800 by 2030 — [operativmm.az, 28 Jan 2026](https://operativmm.az/en/post/baku-bus-fleet-to-be-modernized-by-2030/66473)
- Electric buses: first e-bus Sept 2023 on route 5; 100 e-buses planned for 2024 (Rzayev, March 2024) — [Trend, 7 Mar 2024](https://www.trend.az/azerbaijan/business/3871357.html); 160 BYD K9UD 12 m buses delivered before COP29, inspected by President Aliyev on 31 Oct 2024 — [Gasgoo](https://autonews.gasgoo.com/articles/news/byd-obtains-electric-bus-orders-from-azerbaijans-government-70033838); 166 e-buses in urban service in Aug 2025 — [Report.az](https://report.az/en/amp/infrastructure/number-of-electric-buses-in-azerbaijan-skyrocket-in-august); 167 BYD e-buses electrified 7 routes — [operativmm.az](https://operativmm.az/en/post/bakida-7-marsrut-xetti-elektriklesdirildi/68939); 368 green buses by May 2026 — [APA](https://en.apa.az/wuf13/planned-railway-metro-and-tram-projects-for-passenger-transportation-in-baku-announced-507647)
- Yutong e-buses deployed on route 133 (Chinar Trans LLC) in Dec 2024 — [Global Mass Transit](https://globalmasstransit.net/yutong-buses-deployed-in-baku-azerbaijan/); a Yutong e-bus previously used in Uzbekistan brought to Baku for testing — [Mincom](https://mincom.gov.az/en/media-en/news/one-more-electric-bus-delivered-to-baku-for-testing)
- Local assembly: BYD Azerbaijan Energy Automotive Factory LLC at Sumqayıt Chemical-Industrial Park, >29 million AZN investment, 600 jobs, 250–300 buses/yr, first phase assembles BYD K9UD (82 pax, 350 km range); 2025 contract with BakuBus for e-buses worth 16.6 million AZN; 60 locally assembled BYD buses to Nakhchivan Aug 2026 — [Trend, 28 Sept 2026](https://www.trend.az/business/4228899.html)
- "Albalılıq" electric bus depot (Buzovna, BakuBus) services 167 BYD buses; 365 drivers + 78 admin/technical staff; BakuBus bought 160 e-buses in 2024 and 200 each in 2025 and 2026 — [modern.az](https://modern.az/en/economy/578662/new-bus-depot-commissioned-in-baku-photo/) (secondary; the "200 in 2025 and 2026" claim is not corroborated by AYNA)
- Stated national goal: replace all Baku public transport vehicles with electric by 2030; 200–250 e-buses/yr production from 2026 — [Report.az](https://report.az/en/amp/infrastructure/azerbaijan-keen-to-produce-200-250-electric-buses-annually)
- Driver shortage: Baku.ws links longer intervals in Baku and regions to driver shortages; AYNA's official answer (30 Sept 2025): hiring is the carriers' responsibility under the Labour Code; fleet expansion "creates new jobs"; drivers may work shifts; social protection efforts "ongoing"; AYNA Training Centre offers driver training — [baku.ws, 30 Sept 2025](https://baku.ws/social/avtobus-suruculerinin-maaslari-artirilacaq-resmi-cavab)
- BakuBus driver vacancy (Oct 2026): 1,350 AZN gross, shift work, age ≤65, category D licence, military service completed; extra pay for tenure, uniforms, transport — [bakucity.az](https://bakucity.az/bakubus-driver-vacancy-october-2026/); [ednews.net](https://ednews.net/65-yasadek-sexslere-sans-1350-manat-verilecek)
- AYNA (Oct 2026): large carriers run training; AYNA Training Centre covers safety, communication, ethics, "emotional self-control"; AYNA holds regular meetings with carriers — [1news.az, 2 Oct 2026](https://1news.az/az/news/20261002150642203-Bakida-sinaga-chevrilen-avtobus-seferi-AYNA-sixliq-kondisioner-ve-davranish-qaydalari-ile-bagli-neleri-deyir)
- (older, 2021) BNA imported 320 BMC buses (170 × 9 m, 150 × 12 m) from Turkey — [Report.az, Aug 2021](https://report.az/infrastruktur/bna-paytaxtda-marsrut-sebekesinin-43-nin-dovriyyesi-seffaflasdirilib)

### Inferences
- The "1,000 need replacing" statement (out of ~2,300 daily) implies ~40% of the active fleet is old; with 55% "eco-friendly" (CNG+electric) the remainder is diesel/old.
- AYNA never officially confirms a driver shortage; media and salary-raise questions imply one. Driver availability is a constraint a scheduling/dispatch tool must treat as fixed in the short run.

### Gaps
- No published average fleet age (orta yaş) for Baku buses.
- No AYNA statement quantifying driver shortage or driver headcount citywide.
- Per-operator fleet lists and manufacturer breakdown (BMC, Neoplan, Isuzu, BYD, Yutong) are not published in one place.

---

## Key Question 3 — Technology: control centre, GPS/AVL, apps, real-time displays, BakıKart/cashless, bus lanes and cameras, signal priority

### Takeaway
Every bus relevant to AYNA is GPS-tracked (AYNA MaaS shows live positions and flags when an ETA is schedule-based rather than GPS-based), ~89% of fares are cashless (BakıKart 60% / EMV bank cards 28% / QR 12%), and 114–117 km of bus lanes exist with camera fines of 100 AZN. AYNA is now building a Data Warehouse and a 3-million-AZN "mini" intelligent transport system with an AI agent; the old 110-million-AZN police-run ITS centre is widely regarded as a failure. No evidence of transit signal priority was found.

### Cited Findings
**Control centre / ITS**
- (older, 2011) Nəqliyyatı İntellektual İdarəetmə Mərkəzi (NİİM, Intelligent Transport Management Centre) opened with a 120 m² wall monitor showing 600 images; 192 cameras at 150 intersections plus management data for 2,000 buses; on-board units send bus data to a central server and push to VMS information boards — [president.az](https://president.az/az/articles/view/4048); [az.wikipedia](https://az.wikipedia.org/wiki/N%C9%99qliyyat%C4%B1_%C4%B0ntellektual_%C4%B0dar%C9%99etm%C9%99_M%C9%99rk%C9%99zi)
- That ITS cost 110 million AZN, "largely failed to meet its goals", considered one of Azerbaijan's biggest unsuccessful digitalization projects; some components still run under the Interior Ministry (DİN) — [tezadlar.az, 27 Oct 2026](https://tezadlar.az/mini-niis-hazirlanir-ayna-iflasa-ugramis-evvelki-110-milyonluq-layiheden-ibret-goturecekmi/)
- New AYNA tender (Oct 2026): "mini" NİİS — network diagnostics + AI platform, estimated 3 million AZN incl. 1 year support; analyses bus movements, fare payments, routes to detect delays, congestion, demand, problem corridors; includes an AI agent AYNA staff can query; expert Osman Gündüz questions whether RİNN reviewed it and calls for reuse of old ITS components — [tezadlar.az, 27 Oct 2026](https://tezadlar.az/mini-niis-hazirlanir-ayna-iflasa-ugramis-evvelki-110-milyonluq-layiheden-ibret-goturecekmi/)
- AYNA Data Warehouse (DWH, July 2026): to unify data on public transport, passenger flows, route performance, traffic intensity and vehicle movements; planned AI decision support, AI Assistant, Big Data analytics, predictive analytics. Existing AYNA systems named: AYNA İnformasiya Sistemi, "Rəqəmsal Əkiz" (Digital Twin) and "Taksi Rəqəmsal Əkiz", Biletim.az, AzParking, QR/NFC cashless payment system, AYNA MaaS — [1news.az, 23 July 2026](https://1news.az/az/news/20260723132639398-Azerbaycanda-neqliyyat-sahesinde-Melumat-Anbari-yaradilir)
- AYNA historically published morning/evening peak bulletins from its "Vahid Dispetçer Xidməti"/"Monitorinq Mərkəzi" listing, per corridor, the routes affected and the interval delay (e.g. 28 Nov 2022: 185 buses on 43 routes delayed; 15–18 min extra interval on Bakı-Sumqayıt corridor; 24 Aug 2023: 17–20 min delays on same corridor) — [Report.az, 28 Nov 2022](https://report.az/infrastruktur/bakida-185-avtobusun-hereketi-gecikir); [AZERTAC, 24 Aug 2023](https://azertag.az/xeber/paytaxtda_avtobuslarin_hereket_intervalinda_gecikmeler_musahide_edilir-2739904)

**Apps and real-time information**
- AYNA MaaS launched 5 Dec 2025 (AZ/EN/RU): multimodal planner (bus, metro, rail, scooter, bike), "Nəqliyyat" tab shows approaching vehicles in real time per stop, nearby stops, route search, favourites, trips; registration by phone/SMS; "initial version" — [ayna.gov.az, 5 Dec 2025](https://ayna.gov.az/az/news/ayna-maas-neqliyyatda-mobillik-helli-tetbiqi-istifadeye-verilib-952)
- Update 9 Apr 2026: live bus icons on map improved; ETA improved; complaints handled in-app with replies in "Bildirişlər"; direct call to Call Centre; Google/Apple login; route info incl. carrier, fares, stop list — [ayna.gov.az, 9 Apr 2026](https://ayna.gov.az/az/news/ayna-maasda-funksional-yenilikler-tetbiq-edilib-1066); a further update states that if an ETA is based on the route interval rather than live GPS, the app tells the user — [ayna.gov.az](https://ayna.gov.az/az/news/ayna-maas-tetbiqi-bir-qeder-de-tekmillesdirilib-1021)
- Accuracy complaints (Oct 2026): app sometimes shows an arrival time but the bus never comes, or buses on a route are invisible; expert Rauf Ağamirzəyev: if buses lack GPS they must be equipped; if GPS exists but signal is not transmitted it must be monitored; M-index express routes (running ~2 months) and rerouted 28 May terminals not reflected promptly — [redaktor.az, 26 Oct 2026](https://redaktor.az/news/country/347569-ayna-maas-da-melumatlar-niye-yenilenmir)
- Other apps: BakıKART app ("Where is Bus?", nearby stops, QR ticketing) — [Uptodown listing](https://bak-kart.en.uptodown.com/android); WUF13 guide lists BakıKart, ADY and AYNA MaaS apps for real-time tracking — [APA](https://en.apa.az/wuf13/public-transport-information-released-for-wuf13-participants-503768); official public web map www.map.ayna.gov.az ("Map AYNA") — [WUF13 local transport guide](https://wuf13.az/en/attendee-information/travel-guide/local-transport/); 2GIS and Yandex Maps also show Baku routes — [wander-lush](https://wander-lush.org/baku-to-tbilisi-bus-travel-azerbaijan/)

**Fares / cashless**
- Jan–Aug 2026 cashless share rose from 84.3% to 89.1%; split: BakıKart 60%, EMV bank cards 28%, QR 12% — [Report.az, 28 Sept 2026](https://report.az/en/infrastructure/baku-bus-passenger-trips-reach-335-6-million-in-jan-aug-2026-ayna-says)
- Buses with cashless systems: 1,000 (2022) → 1,800 (Jan 2026) → 1,900 (May 2026); cashless share 85% in Jan 2026 — [Mincom](https://mincom.gov.az/en/media-en/news/roundtable-on-the-2025-state-program-for-the-improvement-of-transport-infrastructure-in-baku-and-surrounding-areas); [APA](https://en.apa.az/wuf13/planned-railway-metro-and-tram-projects-for-passenger-transportation-in-baku-announced-507647)
- (older, 2021) BNA: 43% of route network "turnover made transparent" via BakıKart integration — [Report.az, Aug 2021](https://report.az/infrastruktur/bna-paytaxtda-marsrut-sebekesinin-43-nin-dovriyyesi-seffaflasdirilib)
- Baku Metro accepts bank cards, Apple Pay, Google Pay, foreign cards; QR in app and distance-based fares planned — [Trend, 12 May 2026](https://www.trend.az/azerbaijan/society/4184934.html)

**Bus lanes and enforcement**
- ~114 km of bus lanes across ~40 streets (Jan 2026) — [Mincom](https://mincom.gov.az/en/media-en/news/roundtable-on-the-2025-state-program-for-the-improvement-of-transport-infrastructure-in-baku-and-surrounding-areas); ~117 km on 43 streets (Oct 2026) — [1news.az](https://1news.az/az/news/20261002150642203-Bakida-sinaga-chevrilen-avtobus-seferi-AYNA-sixliq-kondisioner-ve-davranish-qaydalari-ile-bagli-neleri-deyir)
- Fine for entering a bus lane: 100 AZN + 2 penalty points (Road Traffic Law Art. 48); solid line = no entry, dashed = brief entry — [news.az](https://news.az/news/-from-moscow-to-manhattan-smarter-bus-lane-rules-baku-should-steal); [jam-news](https://jam-news.net/bus-lanes-in-baku/); one fine per street even if several cameras record — [bakivaxti.az](https://bakivaxti.az/az/posts/detail/avtobus-zolagina-giren-suruculerin-nezerine-1763495253); a project with cameras on buses recording bus-lane violations, integrated into DYP's system, is described — [bizim.media](https://bizim.media/az/cemiyyet/201387/avtobus-zolagi-ile-hereket-eden-suruculerin-nzrn-100-manat-cerime/); Xocalı prospekti camera-fine dispute, Jan 2025 — [avtosfer.az](https://avtosfer.az/az/qaydalar/parklanmadan-cixanlar-avtobus-zolagini-tapdalayirlar-kamera-cekir-ve-cerime-yazilir-vi%CC%87deo-269973)
- Some lanes operate only in certain hours — [avtosfer.az list](https://avtosfer.az/az/avtoxeber/bakida-avtobus-zolaqlarinin-siyahisi-hansi-saatda-hansi-zolaga-girmek-olar-257354)

### Inferences
- The data for a bunching/headway tool (AVL positions, fare taps per bus, route/stop master data) exists inside AYNA's systems; AYNA's own DWH and "mini NİİS" tenders are essentially the same problem a hackathon team would pitch — a prototype could position itself as a demonstrator for that tender.
- The MaaS app's "ETA based on interval, not GPS" flag indicates real-world AVL gaps (buses offline or not transmitting) — any model must handle missing vehicles.

### Gaps
- No source on transit signal priority (svetofor prioriteti) for buses in Baku.
- No measured ETA accuracy or AVL coverage percentage; no official count of electronic arrival displays at stops.
- Confirmation that cash is fully abolished on all buses was not found (cashless = 89%, so cash still exists).

---

## Key Question 4 — Reliability complaints (bunching, long gaps, overcrowding) and AYNA's explanations/KPIs

### Takeaway
Bunching and 20–60+ minute gaps are a recurring, documented complaint; AYNA's standard explanation is peak-hour traffic on streets without bus lanes, plus demand-matched fleet sizing by carriers. AYNA publishes no on-time or headway-adherence KPI; the only quantified reliability data it has released are ad-hoc corridor delay bulletins and, from Sept 2026, contractually monitored intervals under gross-cost contracts.

### Cited Findings
- Route 49 (Oct 2024): passengers: "they come rarely, and after a long gap 2–3 buses arrive one after another and you can't board any"; AYNA: 26 buses at 7-minute interval run by "Azlənd Plaza" MMC; "during morning and evening peaks, congestion on the streets the route passes causes interval growth"; carrier instructed to adjust bus numbers to passenger flow — [Oxu.az, 23 Oct 2024](https://oxu.az/cemiyyet/bakida-avtobus-intervallari-sernisinleri-narazi-saldi-muvafiq-tapsiriq-verildi)
- Routes 52 and 96 (Sept 2026): waits of nearly an hour, buses arriving full; carrier: 5 buses on 52, 6 on 96, standard interval 15–20 min — [1news.az, 3 Sept 2026](https://1news.az/az/news/20260903104422853-Bakida-avtobus-problemi-52-ve-96-nomreli-marshrutlari-bir-saata-yaxin-gozleyirler)
- Route 37: one bus every 2–3 hours claimed; AYNA: 2 buses at 25–30 min, low demand justifies — [Oxu.az](https://oxu.az/cemiyyet/37-nomreli-avtobusdan-sikayet-gec-gelirler); Route 29: >1 hour 20 min waits — [bakupost.az](https://www.bakupost.az/men-niye-bir-saat-20-deqiqe-avtobus-gozlemeliyem-29-nomreli-marsrutdan-sikayet); Route 114B weekly complaints, broken seats, early end of service — [bakupost.az](https://www.bakupost.az/bakida-bu-avtobusdan-her-hefte-sikayet-gelir-video)
- AYNA general position (Oct 2026): waits of 20–40 min then mass boarding reported; "traffic density, especially where there are no dedicated bus lanes, causes delays and broken intervals"; AYNA "monitors bus loading and adds buses where crowding is high"; complaints via 141 Call Centre and social media; bus camera footage can be requested by police — [1news.az, 2 Oct 2026](https://1news.az/az/news/20261002150642203-Bakida-sinaga-chevrilen-avtobus-seferi-AYNA-sixliq-kondisioner-ve-davranish-qaydalari-ile-bagli-neleri-deyir)
- Daily peak delay bulletins (2022–2023 examples): 185 buses on 43 routes delayed, corridor-level interval delays 10–21 min — [Report.az](https://report.az/infrastruktur/bakida-185-avtobusun-hereketi-gecikir); [AZERTAC](https://azertag.az/xeber/paytaxtda_avtobuslarin_hereket_intervalinda_gecikmeler_musahide_edilir-2739904); other bulletins: 44 buses, 122 buses delayed — [Report.az](https://report.az/infrastruktur/bakida-44-avtobusun-hereketi-gecikir); [Report.az](https://report.az/infrastruktur/bakida-122-avtobusun-hereketi-lengiyir)
- Gross-cost contracts make "schedule adherence, intervals and distance" the measured service indicators, with penalties; aimed explicitly at keeping buses running in low-demand evening periods — [marja.az, 2 Sept 2026](https://marja.az/139391/dasiyicilara-mesafeye-gore-pul-odenilecek-bakida-avtobuslarla-bagli-yeni-sistem-ise-dusdu)
- Bus lanes were built "to ensure regular bus intervals" — [Mincom](https://mincom.gov.az/en/media-en/news/roundtable-on-the-2025-state-program-for-the-improvement-of-transport-infrastructure-in-baku-and-surrounding-areas)
- (older) Bankwatch-type critique applies to Baku too: jam-news argues Baku's bus lanes "fail to facilitate public transport" partly due to design/enforcement — [jam-news](https://jam-news.net/bus-lanes-in-baku/)

### Inferences
- Under per-passenger (net) contracts, carriers had an incentive to bunch at peaks and thin service off-peak; the gross-cost switch removes that incentive but requires AYNA to measure headway adherence — a measurable gap for an AI headway-control / bunching-prediction tool.
- AYNA explains bunching almost exclusively by traffic; no official acknowledgement of operational causes (dispatching, driver breaks, holding strategies).

### Gaps
- No AYNA-published on-time %, headway regularity index, or passenger-wait KPI for 2024–2026.
- No official citywide count of complaints by category for 2025–2026 (only a 2021 BNA monthly figure: 1,256 appeals, 5,856 ALO 141 calls — [Report.az, 2021](https://report.az/infrastruktur/bna-paytaxtda-marsrut-sebekesinin-43-nin-dovriyyesi-seffaflasdirilib)).

---

## Key Question 5 — Open data: GTFS / real-time API, hackathons and developer programs, what a student team can realistically obtain

### Takeaway
AYNA publishes no GTFS, no GTFS-Realtime and no documented public API; but its public web map (map.ayna.gov.az) exposes an unofficial JSON API from which routes, stops and geometry for 201 routes have already been scraped and republished (Kaggle/Zenodo, no licence), and AYNA MaaS exposes live vehicle positions per stop that can in principle be captured. Timetables, AVL history, and ridership are not published. Hackathons exist (TransitHack 2024, Baku Metro 2026) and AYNA has participated as a hackathon client (MoveX prototype).

### Cited Findings
- Preprint: "the data needed [for an evaluation] (timetables, vehicle positions and origin–destination flows) is not published"; author built from "the transport agency's public map, a population grid, OpenStreetMap destinations, metro entries and one operator's daily boardings"; gravity model vs observed boardings Spearman 0.58; redesign saves 0.76 min avg trip or ~12% fewer buses for same travel times — [Samadov, engrXiv, 27 Oct 2026](https://engrxiv.org/preprint/view/8632)
- Repo: snapshot via `data/ayna_scraper.py` → `data/ayna.db` (5 tables; routes, stops, geometry, route–stop assignments; "no timetables, speeds or ridership by stop"); "no published licence", redistributed as research snapshot with attribution to AYNA; AYNA's opendata.az datasets (bicycle lanes 57.1 km/54 streets, 185 parking points) are CC0; Zenodo DOI 10.5281/zenodo.23604370, Kaggle DOI 10.34740/KAGGLE/DSV/21173689 — [ismatsamadov/route_efficiency](https://github.com/ismatsamadov/route_efficiency)
- Digital-twin demo of Baku bus network + bike lanes in a single HTML page — [samirmehd1yev/tranport_digital_twin](https://github.com/samirmehd1yev/tranport_digital_twin); another hackathon repo "baku-mobility-lens" — [GitHub PR](https://github.com/baku-mobilitylens-team/baku-mobility-lens/pull/1)
- MoveX: "hackathon prototype for AYNA and the passengers of Baku"; fleet/crowd data simulated; real inputs: Open-Meteo, Report.az/AZERTAC RSS, Google News RSS, OSM, OSRM; "planned: AYNA GPS and delay feeds"; features: AI dispatch plans, crowd levels from delay (<5 / 5–9 / 10+ min), passenger assistant AZ/EN/RU; stack React 19/Express 5/SQLite/gpt-4o-mini — [IsaMaharramov/moveX](https://github.com/IsaMaharramov/moveX)
- TransitHack 2024, 1–3 Nov 2024, Bakı Ali Neft Məktəbi (BHOS) with Huawei support; >600 participants from 17 countries; tracks: Technology, Research, Urban Design; 12,500 AZN prize fund; winners incl. "125" (multimodal app) and "Urban Rangers" (carpooling); BHOS teams took top 3 — [spark.ru write-up](https://spark.ru/user/216634/blog/227615/transithack-2024-v-baku-buduschee-gorodskogo-transporta-v-rukah-novih-talantov); [bhos.edu.az](https://bhos.edu.az/en/news/2578); [transithack.az](https://transithack.az/) (site unreachable from this session)
- Baku Metro hackathon (spring 2026) at closed İçərişəhər station platform: >1,200 applications, 78 participants; most questions concerned the mobile app; some projects continued and participants recruited — [Trend, 12 May 2026](https://www.trend.az/azerbaijan/society/4184934.html); earlier metro competition with 5,000 AZN prize — [tech.az](https://tech.az/en/posts/a-competition-with-a-prize-of-5-000-azn-begins-in-the-baku-metro-6684)
- AYNA MaaS exposes per-stop real-time approaching vehicles and live map icons (hence a backend vehicle-position feed exists) — [ayna.gov.az](https://ayna.gov.az/az/news/ayna-maas-neqliyyatda-mobillik-helli-tetbiqi-istifadeye-verilib-952); official web map — [map.ayna.gov.az via WUF13 guide](https://wuf13.az/en/attendee-information/travel-guide/local-transport/)
- AYNA's own data stack to pitch into: DWH + Big Data + AI Assistant plans — [1news.az, 23 July 2026](https://1news.az/az/news/20260723132639398-Azerbaycanda-neqliyyat-sahesinde-Melumat-Anbari-yaradilir); 3 million AZN "mini NİİS" with AI agent tender — [tezadlar.az, 27 Oct 2026](https://tezadlar.az/mini-niis-hazirlanir-ayna-iflasa-ugramis-evvelki-110-milyonluq-layiheden-ibret-goturecekmi/)
- Fare-tap data: BakıKart/EMV/QR cover 89% of trips (per-tap data exists at operator/AYNA level) — [Report.az](https://report.az/en/infrastructure/baku-bus-passenger-trips-reach-335-6-million-in-jan-aug-2026-ayna-says)

### Inferences
- Realistic data a student team can get without AYNA cooperation: (1) static routes/stops/geometry for 201 routes from the existing Kaggle/Zenodo snapshot or by re-running the public scraper against map.ayna.gov.az; (2) self-collected AVL by polling the AYNA MaaS / map backend for vehicle positions over days (legal grey zone, no licence); (3) OSM, population grid, metro ridership, weather, news RSS (as MoveX did); (4) AYNA's historic corridor delay bulletins as labelled "delay" events; (5) BakuBus boardings (one operator's daily boardings were used in the preprint).
- Not obtainable publicly: timetables, historic AVL, per-bus tap counts, OD matrices, driver rosters — these would require a data-sharing agreement with AYNA/BakuBus; AYNA's DWH and gross-cost monitoring tool are where they will live.
- No GTFS → Google/Apple Maps rely on third-party data (2GIS, Yandex, Moovit); producing a GTFS feed from the scrape is itself a credible hackathon deliverable.

### Gaps
- Exact endpoints of the AYNA map/MaaS API are not documented in any source read (the scraper code would need to be inspected); their rate limits and terms of use are unknown.
- No evidence of a formal AYNA/RİNN/İRİA open-data or developer program for transit; nothing found on a "Milli Süni İntellekt Mərkəzi" transport initiative.
- transithack.az could not be fetched (domain not resolvable from this session), so official partner list (whether AYNA was a formal partner) is unconfirmed.

---

## Key Question 6 — Comparable regional reforms: Tbilisi, Almaty, Tashkent — what worked

### Takeaway
Tbilisi, Almaty and Tashkent all paired fleet renewal with cashless fares, salaried (not rent-based) drivers, and official real-time apps; Tashkent reports wait times halving (22 → 10–12 min) and a 43.5% ridership jump after banning cash; the Bankwatch critique of Tbilisi — digital tools without a bus-lane network first — is directly relevant to Baku.

### Cited Findings
**Tbilisi**
- Reforms since 2015, actively since 2018: BRT development, fleet upgrade; minibus drivers moved from daily rental fees to monthly salaries with City Hall contracting the management company; critique: the city leaned on digital solutions (app, messaging, digital boards) while the core bus-lane network was not built first — [CEE Bankwatch report, 29 Sept 2022](https://bankwatch.org/wp-content/uploads/2022/09/2022-09-29_Tbilisi-urban-public-transit-reform.pdf)
- June 2023 agreement for 160 × 18 m MAN CNG buses; 50 arrived by March 2024, rest by end-2024 — [Tbilisi City Hall presentation, UNESCAP](https://www.unescap.org/sites/default/d8files/event-documents/Day1_Session4.2_Georgia_Ms.Anano_.pdf)
- Official "Tbilisi Transport" app provides live arrivals and real-time tracking (v3.0.0 March 2024) — [App Store](https://apps.apple.com/us/app/tbilisi-transport/id1437141092); several third-party apps exist (e.g. tbilisi-bus GitHub) — [tbilisi-bus.github.io](https://tbilisi-bus.github.io/)

**Almaty**
- 1,150 buses purchased incl. 550 by private carriers, plans for 1,000 more incl. 100 electric; 450 eco-friendly vehicles tendered and delivered by Nov 2024 — [UNESCAP SUTI Almaty presentation](https://www.unescap.org/sites/default/d8files/event-documents/Day1_Session2.1_SUTI_Ms.Yelena.pdf); [Almaty Development Centre](https://almatydc.kz/en/press/news/realizaciya_reform_transporta_i_mestnoj_policii_g-almaty)
- 8 new routes launched Nov 2024 — [Tengrinews](https://en.tengrinews.kz/kazakhstan_news/8-new-bus-routes-launched-in-almaty-265058/); fare 120 KZT from 3 Aug 2024, cash fully discontinued (Onay card/app; requires Kazakh phone number) — [eurasia.travel forum](https://eurasia.travel/forum/transport-kazakhstan/public-transport-in-almaty/) (secondary)
- Peer-reviewed: people-centred/high-capacity PT interventions align with PM improvements more than road expansion (descriptive, not causal) — [Sustainability 2026](https://doi.org/10.3390/su18052187)

**Tashkent**
- Cash fares banned on city buses from 1 Jan 2025; daily passengers +43.5% in first 10 days vs Jan 2024; payment via transport cards, ATTO app, contactless bank cards, QR; fleet 1,901 buses end-2024 (+28% vs 2022); wait times halved from 22 to 10–12 min in two years; 1,000 new buses planned for 2025 — [Gazeta.uz, 14 Jan 2025](https://www.gazeta.uz/en/2025/01/14/tashkent-buses/)
- 2,000 BYD e-bus plan delayed; experts rate success "only partial"; BRT problems — [Zamin.uz, 4 Nov 2025](https://zamin.uz/en/society/167034-tashkent-transport-system-brt-and-future-problems.html); 202 Yutong e-buses received Oct 2025 — [electrive](https://www.electrive.com/2025/10/16/202-new-electric-yutong-buses-for-uzbekistan)
- ITF/OECD Urban Mobility Improvement Plan for Tashkent — [ITF](https://www.itf-oecd.org/sites/default/files/sipa_uzb_urban_mobility_improvement_plan_eng_0.pdf); ProMobility supported Tashkent bus-operations reform — [pro-mobility.org](https://pro-mobility.org/en/nashi-proektu/supporting-the-reform-of-bus-operations-in-tashkent/)

### Inferences
- Baku in 2026 is roughly where Tbilisi was in 2020–2022 and Tashkent in 2024: fleet renewed, cashless high, app launched, contracts moving to gross-cost; the missing pieces in all three cases were protected right-of-way enforcement and headway management — the operational layer where an AI dispatch/bunching tool adds value.
- Tashkent's cash ban → +43% ridership suggests Baku's remaining ~11% cash trips are a data blind spot a team should flag.

### Gaps
- No published Tbilisi/Almaty/Tashkent evaluation of real-time-app accuracy or headway KPIs after reform.
- No Onay (Almaty) ridership/cost-recovery outcomes found.

---

## Quick reference for the hackathon team (all figures sourced above)
- Network: ~201 routes, 39 carriers, 3,456 stops; 2,300–2,530 buses/day; ~494M pax/yr; 114–117 km bus lanes; 8 "new network" routes + 8 M-express routes live.
- Fleet: ~368 e-buses (BYD K9UD, some Yutong), 55% eco; ~1,000 need replacement; target 2,800 by 2030; BYD assembly in Sumqayıt 250–300/yr.
- Payments: 89.1% cashless (BakıKart 60 / EMV 28 / QR 12); 1,900 buses with contactless.
- Tech: AYNA MaaS (Dec 2025) live positions + ETA with GPS/schedule flag; map.ayna.gov.az public map (scrapable, no licence); DWH in build (Jul 2026); 3M AZN "mini NİİS" + AI agent tender (Oct 2026); old 110M AZN ITS centre under police considered failed.
- Reliability: AYNA blames traffic without bus lanes; no public KPIs; gross-cost contracts (routes 130, 205, 211, 500, 503 from Sept 2026) introduce interval penalties.
- Open data: no GTFS/GTFS-RT; Kaggle/Zenodo static snapshot exists; TransitHack 2024 and Baku Metro 2026 hackathons; MoveX prototype simulated AVL "pending AYNA GPS feed."
