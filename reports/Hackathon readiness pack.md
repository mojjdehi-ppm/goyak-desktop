# Hackathon readiness pack — Ritm

For the "İnsanyönümlü şəhərlər" hackathon (Youth Foundation + National AI Centre). Registration closes **8 November 2026**. Stage I: pitch among 150 participants. Stage II: Startup Demo Day, 10 finalists, 3 winners.

Everything here follows the advisor review of 2 Nov: honest numbers, no cameras, no console integration, no new standby fleet, named models.

---

## 1. Checklist and timeline

### Before 8 Nov (registration)
- [ ] Register every team member: forms.gle/Ecakm2MWMUgXSeZQ6
- [ ] Paste the **Azerbaijani** project text from `Hackathon registration text.md` into the form. Add names and roles.
- [ ] Have one native speaker read the Azerbaijani once.
- [ ] Share the deck and the demo links from their Share menus so teammates can open them.

### This week (before Stage I)
- [ ] **Ride a BakuBus trunk route (2, 7A or 172) and route 61.** Photograph the driver screen yourself. Ask the three questions in section 5. Write down the answers word for word. This is the single most valuable thing you can do.
- [ ] Rehearse the 3-minute pitch (section 2) at least five times, timed.
- [ ] Decide who speaks: one person for the pitch, one on the demo laptop.
- [ ] Record a 90-second screen video of the demo as a backup against Wi-Fi failure.
- [ ] Download `ritm-61.html` to the demo laptop; it runs offline from a file.

### Before Demo Day (if selected)
- [ ] Rehearse the 6-minute demo script (section 3) with the clicks.
- [ ] Update slide 2 with anything the driver told you (quote them).
- [ ] Check whether AYNA's AI-platform tender (bids closed 24 Nov) has a published winner; if so, name it when asked.
- [ ] Print 10 copies of the one-pager (section 8).
- [ ] Bring: laptop, charger, HDMI adapter, phone with the demo video, printed one-pagers.

---

## 2. Stage I pitch, 3 minutes

### English

> Good afternoon. We are [team], and our project is Ritm, which means rhythm.
>
> Every day about 2,500 buses carry Baku's passengers. And every day the same thing happens: you wait twenty minutes, then three buses arrive together. In October, BakuBus's own monitoring centre measured 17-minute intervals on route 2 against a planned 7, and 22 minutes on route 7A against a planned 4. This is called bus bunching. It is physics: a bus that is a little late picks up more passengers, boards longer, and falls further behind, while the bus behind it finds empty stops and catches up.
>
> Baku already has the data. Every bus has GPS and an on-board unit. AYNA's monitoring centre sees the intervals. Dispatchers release buses at the terminals. But nobody closes the loop: nobody computes the gap between two buses in real time and turns it into a timed instruction for the driver. That is Ritm.
>
> Here is how it works. After everyone has boarded and got off, a tablet beside the existing unit shows the driver one line: "earliest departure in 40 seconds", because the bus behind is further away than the bus ahead. It never skips a stop, never shortens boarding, and if an elderly passenger needs time, the advice is simply zero. Behind that one line sits a learning model that predicts when the next bus arrives, and an advisor that tells the dispatcher where a reserve bus would help most.
>
> This method is not new to the world: Washington cut excess waiting by 26 percent, Chicago by 6 to 9 percent with only half the drivers following the advice. It is new to Baku. And since September, interval regularity is a penalised contract KPI here, so carriers now have a financial reason to want it.
>
> We built it on the real route 61, with real stops and streets, and ran five scenarios. We do not claim the ideal one. We claim 5 to 15 percent less excess waiting on a first pilot route, with real drivers.
>
> Our ask is small: one route, one tablet at the terminus, eight weeks, and a data agreement. Thank you.

### Azərbaycanca

> Salam. Biz [komanda]yıq, layihəmiz Ritm adlanır.
>
> Hər gün təxminən 2 500 avtobus Bakı sərnişinlərini daşıyır. Və hər gün eyni şey olur: iyirmi dəqiqə gözləyirsən, sonra üç avtobus birlikdə gəlir. Oktyabrda BakuBus-un öz monitorinq mərkəzi 2 saylı marşrutda planlaşdırılan 7 dəqiqə əvəzinə 17 dəqiqəlik, 7A-da isə planlaşdırılan 4 dəqiqə əvəzinə 22 dəqiqəlik interval ölçüb. Buna avtobus yığılması deyilir. Bu, fizikadır: bir az gecikən avtobus daha çox sərnişin yığır, minmə uzanır, daha da gecikir; arxadakı avtobus isə boş dayanacaqlar tapır və ona çatır.
>
> Bakıda məlumat artıq var. Hər avtobusda GPS və bort cihazı var. AYNA-nın monitorinq mərkəzi intervalları görür. Dispetçerlər avtobusları son dayanacaqdan buraxır. Amma heç kim dövrəni qapatmır: heç kim iki avtobus arasındakı məsafəni real vaxtda hesablayıb sürücüyə vaxtlı tövsiyəyə çevirmir. Ritm məhz budur.
>
> Belə işləyir. Hamı minib-düşəndən sonra mövcud cihazın yanındakı planşet sürücüyə bir sətir göstərir: "ən tez 40 saniyədən sonra hərəkət et", çünki arxadakı avtobus qabaqdakından daha uzaqdadır. Heç bir dayanacaq keçilmir, minmə qısaldılmır, yaşlı sərnişin vaxt istəyirsə, tövsiyə sadəcə sıfır olur. Bu bir sətrin arxasında növbəti avtobusun nə vaxt gələcəyini proqnozlaşdıran öyrənən model və ehtiyat avtobusun hara göndərilməsini dispetçerə məsləhət görən modul dayanır.
>
> Bu üsul dünya üçün yeni deyil: Vaşinqton artıq gözləməni 26 faiz, Çikaqo sürücülərin yalnız yarısı əməl etdiyi halda 6–9 faiz azaldıb. Bakı üçün yenidir. Sentyabrdan isə interval nizamlılığı burada cərimə ilə ölçülən müqavilə göstəricisidir, yəni daşıyıcıların bunu istəməyə maliyyə səbəbi var.
>
> Biz bunu real 61 saylı marşrutda, real dayanacaq və küçələrlə qurduq və beş ssenari sınadıq. İdeal ssenarini iddia etmirik. İlk pilot marşrutda, real sürücülərlə, artıq gözləmənin 5–15 faiz azalmasını iddia edirik.
>
> İstəyimiz kiçikdir: bir marşrut, son dayanacaqda bir planşet, səkkiz həftə və məlumat razılaşması. Təşəkkür edirik.

---

## 3. Demo Day script, 6 minutes, with clicks

Open `ritm-61.html` before you start. Leave it on **No control**, speed 30×. It opens already bunched after a 1-hour warm-up.

| Time | Say | Do |
|---|---|---|
| 0:00 | "This is route 61, Azadlıq to 9-cu mkr, 53 real stops on real Baku streets. The buses and passengers are simulated; the route is not." | Point at the map. Zoom once on Tibb Universiteti. |
| 0:40 | "Look at the gaps strip: 2 minutes, 2 minutes, then 25. And the score: irregularity 1.9, 'bunched'. This is what BakuBus measured on routes 2 and 7A last month." | Point to the gap strip and the CV tile. |
| 1:10 | "Today's practice is this: a dispatcher releases buses on time at the terminal. Watch: it helps a little, and it does not stop bunching, because the damage is done en route." | Click **Today: terminal dispatch**. Let it run 20 s. |
| 1:40 | "Now Ritm. Scenario: Chicago-like, half the drivers comply, GPS every 60 seconds, one bus not reporting. This is the honest setting." | Check the scenario select shows Chicago-like. Click **Ritm on**. |
| 2:10 | "The teal buses are holding after boarding has finished. Here is the driver's tablet: 'earliest departure in 38 s, gap ahead 2:10, gap behind 7:40'. Nothing else changes for the driver: every stop served, boarding never cut." | Select a holding bus in **Show bus**. |
| 2:50 | "Watch the score fall: irregularity from 1.9 towards 0.5, bunched from 70 percent towards 5. Waits from 21 minutes to about 7." | Point to the KPI tiles and the chart. |
| 3:20 | "Now Baku's real problem: a bus breaks down." | Click **Take a bus out of service**. |
| 3:40 | "A gap opens. Ritm re-spaces the remaining buses, and the add-a-bus advisor ranks the options for the reserve bus the carrier already keeps at the terminal: drive time, passengers helped. The dispatcher picks one." | Wait for the card, click **Send** on option 1. |
| 4:20 | "A jam on Ceyhun Səlimov street. The street turns red from bus speeds alone, no cameras. The crowding index moves the metro stops to the top." | Click **Jam on Ceyhun Səlimov küç.** Point to the crowding index. |
| 4:50 | "And this is the table we will be judged on. Ideal: 5 minutes. We do not claim it. Chicago-like: 7 minutes. Pessimistic: 18, almost no gain. Our claim: 5 to 15 percent on a first pilot route." | Point to **What we claim**. |
| 5:20 | "What we need: one route, a tablet at one terminus, eight weeks, a data agreement with BakuBus or AYNA. The route is real. The data plumbing exists. The loop is missing. We close it." | Last slide. |

If the laptop fails: play the 90-second video from the phone and talk over it.

---

## 4. Judge Q&A, with answers

**"AYNA already tracks buses. Isn't this done?"**
Tracking yes; control no. The monitoring centre warns carriers after the fact and dispatchers release buses at terminals. No public source shows anyone computing the gap between two buses in real time and giving the driver a timed instruction, and nobody has published a regularity figure for a single route. The proof is in the numbers: with all that tracking, routes 2 and 7A still run 17 to 22-minute gaps against a 4 to 8-minute plan.

**"The driver already has a screen. What's new?"**
That screen is a Kentkart unit built for fare validation and timekeeping; it shows deviation from the timetable. On a busy route in traffic the timetable is the wrong reference: two buses can both be "on time" and 30 seconds apart. Ritm compares the gap ahead with the gap behind. We run on a separate tablet, we do not claim to integrate with that unit, and the advice shows only while the bus is stationary.

**"Where is the AI?"**
In three named places. A LightGBM model forecasts when the bus behind arrives; we show its error against the timetable estimate. OR-Tools ranks add-a-bus options. The instruction itself comes from a transparent linear rule, on purpose: that is what has worked in the field. Reinforcement learning is an offline experiment in our appendix.

**"What about routes with 5 buses every 15 minutes?"**
Holding does not pay there; it adds waiting. Headway control is for routes with 10-minute or better headways, which in Baku is the 30 to 40 trunk routes with 20 or more buses. For 15 to 20-minute routes the right tool is an accurate timetable and honest arrival information, which is a separate module on our roadmap.

**"Drivers won't follow the advice."**
Correct, not all of them. Chicago measured 35 to 57 percent compliance and still cut waits 6 to 9 percent. Our simulation's Chicago-like scenario assumes 50 percent and shows the gain survives; our pessimistic scenario shows that if only a few drivers comply, the gain disappears. That is why we start at the terminal, where a dispatcher stands, and why we ask drivers what would stop them before we design the screen.

**"Passengers will complain that the bus is idling."**
They will, if nobody tells them. Stockholm measured about 20 seconds of extra in-vehicle time per rider. Our holds are short, 30-second steps with a 60 to 120-second cap, and the pilot includes an on-board announcement: "waiting briefly to keep buses evenly spaced".

**"Holding makes trips longer. Doesn't that cost buses?"**
A little. Short, capped holds add a few percent to the lap. On a bunched route that slack is already lost to the bunching itself. We measure the trade-off in the pilot and stop if the lap grows more than the waiting falls.

**"Some buses don't report GPS. Then what?"**
Then Ritm gives no advice on a guess. If the bus behind is silent, the controller falls back to restoring the planned gap ahead only, capped at 60 seconds, and flags the data gap to the dispatcher. The demo shows this: one bus is marked "not reporting".

**"Your simulation shows 70 percent gains. Really?"**
In the ideal row, yes, and we do not claim it. We show all five rows. Our claim for a first pilot is 5 to 15 percent less excess waiting, the range real cities achieved with real drivers.

**"What data do you actually have?"**
Real: route 61's stops and shape from AYNA's public map, streets and places from OpenStreetMap, open ridership tables. Ours: positions read from the public AYNA app for the pilot route and manual stop counts. Synthetic and labelled: the timetable, speeds and passengers in the simulation. A pilot runs on the official feed under a data agreement.

**"AYNA is buying an AI platform this month. Aren't you redundant?"**
That tender buys diagnostics: occupancy, demand peaks, slow zones, an analyst's question-answering agent. It analyses the network. It does not advise a driver or place a bus. Ritm is the operations module that would sit on top of whatever platform AYNA buys, using the same GPS and payment data.

**"Who pays?"**
Two customers. Carriers first: since September, uneven intervals cost them penalty money under gross-cost contracts, and Cabinet Decision 250 lists dispatcher services and software as reimbursable costs, so a Ritm fee passes through their per-km rate. AYNA second, as a module for its platform. The path is İRİA's Openwave programme and a quotation request under 100,000 manats.

**"Why not just more buses or bus lanes?"**
Both help and both are AYNA's job; we support them. But a broken-down bus or a jam will always open a gap, and only control closes it with the fleet you have. Chicago got its gains while 15 to 25 percent of trips were cancelled. That is Baku's situation.

**"The reserve buses: do they exist?"**
Carriers already keep reserve vehicles per route; AYNA's tenders require them. We do not propose a new standby fleet. Our advisor ranks where an existing reserve would help most, and the dispatcher decides. It is validated in simulation only, and we say so.

**"What is your pilot?"**
One route, eight weeks. Week one measures today's regularity from GPS. Then Ritm on, terminal first, a tablet at one terminus and in a few buses. Success: 5 to 15 percent less excess waiting, bunched share under 5 percent, first published CV for a Baku route. Candidate hosts: BakuBus on route 2 or 7A, then Xaliq Faiqoğlu on 61.

**"Privacy?"**
No driver rankings or individual scoring. Passenger data only in aggregate. Hosting in Azerbaijan. The operator is the data controller under Law 998-IIIQ.

**"What happens after the hackathon?"**
Most hackathon projects stop within a week; we know the number is 3.5 percent alive at five months. That is why our ask is the pilot itself, not a prize: a route, a tablet, a data agreement. Demo Day is the start of the eight weeks.

**"Have you talked to a driver?"**
[Answer with what you learned on your ride. If you have not ridden yet, do it before Demo Day. This question will come.]

---

## 5. Driver and dispatcher interview sheet

Ride a BakuBus route (2, 7A or 172) and route 61. Sit near the front. Ask politely at the terminus, not while driving. Record the answers in the driver's own words.

| Ask | Who | Why it matters |
|---|---|---|
| "On your screen, what do '+1:54' and the clock number mean? Who looks at them, and what happens to you if you run early or late? When the bus ahead is right in front of you, what do you do today?" | Driver | Tells you whether the screen enforces the timetable, and whether a second instruction would conflict with it. |
| "When intervals break on this route at 18:00, what exactly do you do: call the driver, hold at the terminus, add a bus? How long does it take? How many buses and drivers are actually spare at that hour?" | Team leader / dispatcher | Tells you whether terminal holding already happens, through which channel, and whether reserve buses exist in practice. |
| "If a tablet told you to wait 60 seconds at Gənclik after everyone had boarded, what would stop you: your break, the timetable, passengers shouting, the controller, a fine?" | Both | Tells you the main compliance barrier in Baku, so the pilot design addresses it. |

Also note: the make of the on-board unit (look for a Kentkart logo), whether the bus has a second screen, and whether the dispatcher ever radios the driver.

---

## 6. Numbers to cite, and numbers never to cite

**Cite (sourced in the research reports):**
- Baku: ~494M bus passengers a year; ~2,500 buses a day; ~209 route codes; mean ~12 buses per route; 30–40 trunk routes with ≥20 buses.
- Measured gaps: route 2, 17 min vs 7–8 planned; route 7A (35 buses), 22 min vs 4–5 planned (BakuBus monitoring centre, Oct 2026). Routes 52/96: waits near an hour (Sept 2026).
- Field results: WMATA −26% excess wait; CTA −6 to −9% average wait (−20% in one peak) at 35–57% compliance with 15–25% trips cancelled; Stockholm −10% wait; Singapore −15 to −25% excess wait.
- Terminal departures: a 0–1-minute departure gap gives ~70% chance of downstream bunching (TriMet data).
- Compliance: 35% terminal, 57% en route (Chicago).
- Policy: gross-cost contracts from Sept 2026 on routes 130, 205, 211, 500, 503; Cabinet Decision 250 lists dispatcher service and software as reimbursable.
- Our claim: 5–15% less excess waiting on a first pilot route; bunched share under 5% as a target.

**Never cite:**
- "−70% waiting" from our ideal simulation as a claim.
- Dubai "+50% punctuality" or "−68% early departures"; Shenzhen "−33% waiting" (no published method).
- Vendor on-time figures presented as headway results.
- "Surtrac −25–40%" or Google Green Light "up to 30%" (traffic signals, stale baselines; not our domain).
- Any claim of integration with the Kentkart unit, access to AYNA GPS or BakıKart data, or a 20-bus standby fleet.
- Financial motives of drivers as a cause of bunching.

---

## 7. Demo operating guide

- **File:** `demo/live61/ritm-61.html`. Runs offline from a file in Chrome or Edge. Also published as a private link; share it from the page's Share menu before judges can open it.
- **Controls:** No control / Today: terminal dispatch / Ritm on. Scenario select: Ideal, Chicago-like (default), Pessimistic. Buttons: take a bus out of service, jam, university rush. Speed 10/30/60×. Reset.
- **Map:** drag to pan, + and − to zoom, click a bus to show its tablet.
- **Panels:** Service quality (CV grade, average wait, bunched, long gaps, advice followed, buses reporting), driver tablet, add-a-bus advisor, "What we claim" table, gaps strip, crowding index, event log.
- **Reset before you present.** The page warms up 1 hour of bunching on load.
- **Backup:** record 90 s of the sequence in section 3 as a video. Keep it on the phone.

---

## 8. One-page leave-behind (print)

**Ritm — AI headway management for Baku buses**

*The problem.* Baku's buses bunch: in Oct 2026 BakuBus measured 17-minute gaps on route 2 (planned 7–8) and 22 minutes on 7A (planned 4–5). Bunching is a feedback loop that a timetable cannot correct.

*What exists.* GPS and a Kentkart on-board unit in every route bus; AYNA's monitoring centre; terminal dispatchers; interval as a penalised KPI under gross-cost contracts since Sept 2026.

*What is missing.* Nobody computes the gap between two buses in real time and turns it into a timed instruction for the driver. Nobody has published a regularity figure for a single route.

*Ritm.* (1) Headway Guardian: after boarding is complete, a tablet beside the existing unit shows "earliest departure in 40 s"; terminal first, then metro transfer stops; never skips a stop, never shortens boarding; 60–120 s cap; no advice when the bus behind is silent. (2) Crowding index: low/medium/high per stop from headways and place types; becomes a forecast with stop-level tap counts. (3) Add-a-bus advisor: ranks options for the reserve buses carriers already hold; the dispatcher decides.

*Evidence.* WMATA −26% excess wait; Chicago −6 to −9% at 35–57% driver compliance and 15–25% cancelled trips; Singapore −15 to −25%.

*Our claim.* 5–15% less excess waiting on a first pilot route at real compliance. Bunched share under 5% as a target. Simulation on the real route 61 shows the full range, from ideal (5-minute waits) to pessimistic (no gain).

*AI, named.* Linear headway rule in the cab; LightGBM arrival forecasts with error shown; OR-Tools assignment; RL offline only.

*Business.* Carriers pay from avoided penalties (dispatcher service and software are reimbursable under Cabinet Decision 250); AYNA as a platform module. Path: İRİA Openwave, RFQ under 100,000 AZN.

*Ask.* One route (BakuBus 2 or 7A, or Xaliq Faiqoğlu 61), one terminus tablet, eight weeks, a data agreement.

*Team.* [names, roles, contact]
