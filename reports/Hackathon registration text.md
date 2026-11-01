# Registration text — "İnsanyönümlü şəhərlər" hackathon

Copy the block for each form field you are asked. The Azerbaijani version comes first, then the English one. Fill in the team section before you submit.

---

## Azərbaycanca

**Layihənin adı:**
Ritm — Bakı avtobusları üçün süni intellekt əsaslı interval, sərnişin axını və ehtiyat park idarəetmə sistemi

**Qısa təsvir (1 cümlə):**
Ritm AYNA-nın mövcud GPS məlumatları üzərində işləyərək avtobusların birlikdə gəlməsinin qarşısını alır, sıxlığı əvvəlcədən proqnozlaşdırır və ehtiyat avtobusları lazım olan yerə vaxtında yönləndirir.

**Problem:**
Bakıda avtobus ən çox istifadə olunan ictimai nəqliyyat növüdür: ildə təxminən 494 milyon sərnişin daşınır, gündə 2 500-ə yaxın avtobus xəttə çıxır. Bununla belə, eyni marşrutun avtobusları tez-tez 2–3 ədəd birlikdə gəlir, ardınca isə uzun fasilə yaranır. Sərnişinlər 49, 52 və 96 saylı marşrutlarda bu barədə şikayət ediblər. Bu "avtobus yığılması" (bus bunching) tıxacdan, dayanacaqlarda sərnişin sayının qeyri-bərabər olmasından və avtobus çatışmazlığından yaranır. Nəticədə insanlar uzun gözləyir, avtobuslar həddindən artıq dolu olur, çoxları şəxsi avtomobilə keçir və tıxac daha da artır.

**Həll:**
Ritm AYNA-nın mövcud GPS/IoT məlumatları üzərində işləyən proqram təminatıdır və üç moduldan ibarətdir:
1. **İnterval Qoruyucusu.** Avtobusların bir-birinə yaxınlaşmağa başladığını real vaxtda aşkar edir və dispetçerə, sürücüyə sadə tövsiyə verir: bütün sərnişinlər minib-düşdükdən sonra "ən tez 40 saniyədən sonra hərəkət et". Avtobus heç bir dayanacağı keçmir, minmə-düşmə vaxtı qısaldılmır. Yaşlı və ya əlilliyi olan sərnişin uzun müddət minirsə, sistem gözləmə tövsiyəsini avtomatik sıfırlayır.
2. **Sərnişin axını proqnozu.** Hansı dayanacaqların 15–60 dəqiqə sonra sıxlaşacağını proqnozlaşdırır. Bunun üçün keçmiş sərnişin məlumatlarından, metro çıxışlarından, dayanacaq ətrafındakı məktəb, universitet və ticarət mərkəzlərindən, saat, hava və tədbir məlumatlarından istifadə edir. Avtobusların öz gecikmələri canlı tıxac sensoru rolunu oynayır, ona görə də kamera lazım deyil.
3. **Ehtiyat park idarəetməsi.** 3–4 məntəqədə saxlanılan təxminən 20 ehtiyat avtobus və mikroavtobusdan hansının, hara və nə vaxt göndəriləcəyini tövsiyə edir. Son qərarı dispetçer verir.

**Süni intellektin rolu:**
- LightGBM modelləri avtobusun gəliş vaxtını və dayanacaqlardakı sərnişin axınını proqnozlaşdırır.
- Beynəlxalq təcrübədə sınanmış interval idarəetmə qaydası təhlükəsizlik məhdudiyyətləri ilə tətbiq olunur.
- Ehtiyat avtobusların optimal təyinatı OR-Tools ilə hesablanır.
- Bütün tövsiyələr izah edilə biləndir: dispetçer hər qərarın səbəbini görür.

**Beynəlxalq sübut:**
Oxşar interval idarəetməsi Vaşinqtonda (WMATA) artıq gözləmə vaxtını 26% azaldıb. Çikaqoda (CTA) reysların 15–25%-i ləğv edildiyi vaxtda belə gözləmə 6–20% azalıb. Sinqapurda bu göstərici 15–25% təşkil edib.

**Niyə indi:**
2026-cı ilin sentyabrından 130, 205, 211, 500 və 503 saylı marşrutlarda brutto müqavilələr tətbiq olunur, yəni interval intizamı maliyyə göstəricisinə çevrilib. Ritm əlavə avadanlıq tələb etmir: mövcud GPS və hər avtobusda bir planşet və ya telefon kifayətdir.

**İnsanyönümlülük:**
- Gözləmə daha qısa olur və daha dəqiq proqnozlaşdırılır.
- Avtobuslarda sıxlıq azalır.
- Yaşlılar və əlilliyi olan şəxslər təhlükəsiz minir, çünki sistem sürücünü heç vaxt tələsdirmir.
- İnsanlar şəxsi avtomobildən ictimai nəqliyyata keçir.

**Gözlənilən nəticə (bir marşrutda 8 həftəlik pilot):**
- Artıq gözləmə vaxtı ən azı 15% azalır.
- Birlikdə gələn avtobusların payı 5%-dən aşağı düşür.
- Sürücülər üçün əlavə avadanlıq və ya mürəkkəb təlim tələb olunmur.

**Hazırkı mərhələ:**
Konsepsiya və arxitektura hazırdır. Bakı statistikası ilə kalibrlənmiş simulyasiya modeli və dispetçer paneli üzərində iş gedir.

**Komanda:**
[Ad Soyad — rol]
[Ad Soyad — rol]
[Ad Soyad — rol]

---

## English

**Project name:**
Ritm — AI-based headway, passenger-flow and reserve-fleet management for Baku buses

**One-line summary:**
Ritm runs on AYNA's existing GPS data. It prevents buses from arriving in bunches, predicts crowding in advance and sends reserve buses where they are needed in time.

**Problem:**
Buses are Baku's most-used public transport: about 494 million passengers a year and roughly 2,500 buses in service each day. Yet buses on the same route often arrive two or three at a time, followed by a long gap. Passengers have complained about this on routes 49, 52 and 96. This "bus bunching" is caused by traffic, uneven passenger numbers at stops and the shortage of buses. As a result people wait longer, buses are overcrowded, many switch to private cars, and congestion gets worse.

**Solution:**
Ritm is software that runs on AYNA's existing GPS/IoT data. It has three modules:
1. **Headway Guardian.** Detects in real time when buses start to bunch and gives the dispatcher and driver one simple instruction: once all passengers have boarded and alighted, "depart no earlier than 40 s from now". The bus never skips a stop and boarding is never cut short. If an elderly or disabled passenger needs extra time, the hold is automatically set to zero.
2. **Passenger-flow forecast.** Predicts which stops will become crowded 15–60 minutes ahead. It uses past passenger data, metro exits, the schools, universities and shopping centres around each stop, the time of day, weather and events. The buses' own delays act as a live traffic sensor, so no cameras are needed.
3. **Reserve-fleet management.** Keeps about 20 reserve buses and minibuses at 3–4 hubs and recommends which one to send, where and when. The dispatcher makes the final decision.

**Role of AI:**
- LightGBM models forecast bus arrival times and passenger flow at stops.
- A headway-control rule already proven in other cities runs with safety guardrails.
- OR-Tools works out the best assignment of reserve buses.
- Every recommendation can be explained: the dispatcher sees the reason behind each one.

**International evidence:**
Similar headway control cut excess waiting time by 26% in Washington DC (WMATA). In Chicago (CTA) it cut waits by 6–20% even while 15–25% of trips were cancelled. In Singapore the reduction was 15–25%.

**Why now:**
From September 2026, gross-cost contracts apply on routes 130, 205, 211, 500 and 503, so keeping even intervals now affects operators financially. Ritm needs no new hardware: the existing GPS plus one tablet or phone per bus is enough.

**Human-centred impact:**
- Waits become shorter and easier to predict.
- Buses are less overcrowded.
- Elderly and disabled passengers can board safely, because the system never rushes the driver.
- People switch from private cars to public transport.

**Expected results (8-week pilot on one route):**
- Excess waiting time falls by at least 15%.
- The share of bunched buses drops below 5%.
- Drivers need no extra hardware or complex training.

**Current stage:**
The concept and architecture are complete. Work is under way on a simulation model calibrated to Baku statistics and on the dispatcher dashboard.

**Team:**
[Name Surname — role]
[Name Surname — role]
[Name Surname — role]
