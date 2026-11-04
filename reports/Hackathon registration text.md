# Registration text — "İnsanyönümlü şəhərlər" hackathon

Copy the block for each form field you are asked. The Azerbaijani version comes first, then the English one. Fill in the team section before you submit. Revised 4 Nov 2026 after the advisor review: honest result range, no camera, no new standby fleet, no console-integration claim.

---

## Azərbaycanca

**Layihənin adı:**
Ritm — Bakı avtobusları üçün süni intellekt əsaslı interval idarəetmə sistemi

**Qısa təsvir (1 cümlə):**
Ritm AYNA-nın və daşıyıcıların artıq topladığı GPS məlumatları üzərində işləyərək avtobusların birlikdə gəlməsinin qarşısını alır, hansı dayanacaqların sıxlaşacağını göstərir və ehtiyat avtobusun hara göndərilməsini dispetçerə tövsiyə edir.

**Problem:**
Bakıda avtobus ən çox istifadə olunan ictimai nəqliyyat növüdür: ildə təxminən 494 milyon sərnişin daşınır, gündə 2 500-ə yaxın avtobus xəttə çıxır. Bununla belə, eyni marşrutun avtobusları tez-tez 2–3 ədəd birlikdə gəlir, ardınca isə uzun fasilə yaranır. 2026-cı ilin oktyabrında BakuBus-un öz monitorinq mərkəzi 2 saylı marşrutda planlaşdırılan 7–8 dəqiqə əvəzinə 17 dəqiqəlik, 7A marşrutunda isə planlaşdırılan 4–5 dəqiqə əvəzinə 22 dəqiqəlik intervallar qeydə alıb. Bu "avtobus yığılması" (bus bunching) tıxacdan, dayanacaqlarda sərnişin sayının qeyri-bərabər olmasından və avtobus çatışmazlığından yaranır. Bakıda avtobuslar artıq GPS ilə izlənilir və intervallar ölçülür; lakin heç bir açıq mənbə iki avtobus arasındakı məsafənin real vaxtda hesablanıb sürücüyə vaxtlı tövsiyə kimi çatdırıldığını göstərmir. Düzəlişlər ya əvvəlcədən planlaşdırma (avtobus sayı), ya son dayanacaqda dispetçerin buraxması, ya da sonradan daşıyıcıya xəbərdarlıq səviyyəsindədir.

**Həll:**
Ritm mövcud GPS məlumatları üzərində işləyən proqram təminatıdır və üç moduldan ibarətdir:
1. **İnterval Qoruyucusu (əsas modul).** Avtobusların bir-birinə yaxınlaşmağa başladığını real vaxtda aşkar edir və əvvəlcə son dayanacaqda, sonra isə metro kimi böyük keçid dayanacaqlarında sadə tövsiyə verir: bütün sərnişinlər minib-düşdükdən sonra "ən tez 40 saniyədən sonra hərəkət et". Avtobus heç bir dayanacağı keçmir, minmə-düşmə vaxtı qısaldılmır. Yaşlı və ya əlilliyi olan sərnişin uzun müddət minirsə, tövsiyə olunan gözləmə avtomatik sıfırlanır. Tövsiyə yalnız avtobus dayandıqda göstərilir; maksimum 60–120 saniyədir; arxadakı avtobus GPS göndərmirsə, tövsiyə verilmir.
2. **Sıxlıq indeksi.** Hansı dayanacaqların növbəti avtobusa qədər sıxlaşacağını aşağı / orta / yüksək kimi göstərir. İntervallardan, dayanacaq ətrafındakı məktəb, universitet, poliklinika və metrodan, saatdan istifadə edir. Avtobusların öz gecikmələri canlı tıxac sensoru rolunu oynayır; kamera istifadə olunmur. AYNA dayanacaq üzrə ödəniş məlumatlarını paylaşdıqda bu indeks proqnoza çevrilir.
3. **Əlavə avtobus məsləhətçisi.** Böyük fasilə yaranmağa başlayanda daşıyıcının artıq saxladığı ehtiyat avtobuslardan hansının, haradan və hara göndərilməsinin daha faydalı olduğunu dispetçerə sıralanmış variantlar kimi göstərir. Son qərarı dispetçer verir. Bu modul hələlik simulyasiyada yoxlanılıb.

**Süni intellektin rolu:**
- Sürücüyə verilən tövsiyə şəffaf, beynəlxalq təcrübədə sahədə sınanmış xətti interval qaydası ilə hesablanır (Vaşinqton, Çikaqo, Stokholm, Atlanta).
- LightGBM modeli arxadakı avtobusun gəliş vaxtını proqnozlaşdırır; onun xətası cədvəl əsaslı proqnozla müqayisədə göstərilir.
- Ehtiyat avtobus variantları OR-Tools ilə hesablanır.
- Gücləndirilmiş öyrənmə (RL) yalnız oflayn təcrübədir; kabinədə deyil.
- Bütün tövsiyələr izah edilə biləndir: dispetçer hər qərarın səbəbini görür.

**Beynəlxalq sübut (sahə nəticələri):**
Vaşinqtonda (WMATA) artıq gözləmə vaxtı 26% azalıb. Çikaqoda (CTA) sürücülərin yalnız 35–57%-i tövsiyəyə əməl etdiyi və reysların 15–25%-i ləğv edildiyi halda belə orta gözləmə 6–9% azalıb. Sinqapurda bu göstərici 15–25% təşkil edib.

**Niyə indi:**
2026-cı ilin sentyabrından brutto müqavilələr tətbiq olunur: interval intizamı daşıyıcı üçün maliyyə göstəricisinə çevrilib, dispetçer xidməti və proqram təminatı xərcləri isə ödənilən xərc maddələridir (Nazirlər Kabinetinin 250 saylı qərarı). Ritm əlavə avadanlıq tələb etmir: pilot avtobuslarda mövcud bort cihazının yanında bir planşet kifayətdir.

**İnsanyönümlülük:**
- Gözləmə daha qısa və daha proqnozlaşdırıla bilən olur.
- Avtobuslarda sıxlıq azalır.
- Yaşlılar və əlilliyi olan şəxslər təhlükəsiz minir, çünki sistem sürücünü heç vaxt tələsdirmir.
- Sürücü reytinqi aparılmır; sərnişin məlumatları yalnız ümumiləşdirilmiş şəkildə istifadə olunur.

**Gözlənilən nəticə (bir marşrutda 8 həftəlik pilot):**
- Artıq gözləmə vaxtı real sürücü davranışında 5–15% azalır (simulyasiyada daha çox, lakin biz sahə rəqəmlərini əsas götürürük).
- Birlikdə gələn avtobusların payı 5%-dən aşağı düşür (hədəf).
- Bakıda ilk dəfə bir marşrut üçün interval nizamlılığı göstəricisi (CV, artıq gözləmə) dərc edilir.

**Hazırkı mərhələ:**
Real 61 saylı marşrutun (AYNA xəritəsi, OpenStreetMap) üzərində işləyən simulyasiya, dispetçer paneli və sürücü planşeti prototipi hazırdır. Dörd ssenari (nəzarətsiz, bugünkü son dayanacaq dispetçeri, ideal, Çikaqo-bənzər, pessimist) müqayisə edilib.

**Komanda:**
[Ad Soyad — rol]
[Ad Soyad — rol]
[Ad Soyad — rol]

---

## English

**Project name:**
Ritm — AI headway management for Baku buses

**One-line summary:**
Ritm runs on the GPS data AYNA and carriers already collect. It stops buses from arriving in bunches, shows which stops will crowd, and advises the dispatcher where to send a reserve bus.

**Problem:**
Buses are Baku's most-used public transport: about 494 million passengers a year and roughly 2,500 buses in service each day. Yet buses on the same route often arrive two or three at a time, followed by a long gap. In October 2026 BakuBus's own monitoring centre recorded 17-minute intervals on route 2 against a planned 7–8, and 22 minutes on route 7A against a planned 4–5. This "bus bunching" is caused by traffic, uneven passenger numbers at stops and the shortage of buses. Baku already tracks its buses and measures intervals; but no public source shows anyone computing the gap between two buses in real time and turning it into a timed instruction for the driver. Every documented correction is planning-level (more buses), terminal-level (a dispatcher releasing departures) or after the fact (a warning to the carrier).

**Solution:**
Ritm is software that runs on existing GPS data. It has three modules:
1. **Headway Guardian (core).** Detects in real time when buses start to bunch and gives one simple instruction, first at the terminal and then at major transfer stops such as metro stations: once all passengers have boarded and alighted, "depart no earlier than 40 s from now". The bus never skips a stop and boarding is never cut short. If an elderly or disabled passenger needs extra time, the hold is automatically set to zero. Advice appears only while the bus is stationary, is capped at 60–120 s, and is withheld when the bus behind is not reporting GPS.
2. **Crowding index.** Flags which stops will crowd before the next bus arrives, as low / medium / high, from headways, the schools, universities, clinics and metro exits around each stop, and the time of day. The buses' own delays act as a live traffic sensor; no cameras are used. The index becomes a forecast when AYNA shares stop-level tap-in counts.
3. **Add-a-bus advisor.** When a large gap starts to form, it ranks options for the reserve buses the carrier already holds: which vehicle, from which terminal, where to join. The dispatcher makes the final decision. This module has so far been validated only in simulation.

**Role of AI:**
- The instruction to the driver comes from a transparent linear headway rule that has worked in the field (Washington, Chicago, Stockholm, Atlanta).
- A LightGBM model forecasts the arrival time of the bus behind; its error is shown against the timetable-based estimate.
- OR-Tools ranks the add-a-bus options.
- Reinforcement learning is an offline experiment only, not in the cab.
- Every recommendation can be explained: the dispatcher sees the reason behind each one.

**International evidence (field results):**
Washington DC (WMATA) cut excess waiting time by 26%. Chicago (CTA) cut average waits by 6–9% with only 35–57% of drivers following the advice and 15–25% of trips cancelled. Singapore cut excess waiting by 15–25%.

**Why now:**
From September 2026, gross-cost contracts make interval regularity a financial indicator for carriers, and dispatcher-service and software costs are reimbursable cost items (Cabinet Decision No. 250). Ritm needs no new hardware: a tablet beside the existing on-board unit on pilot buses is enough.

**Human-centred impact:**
- Waits become shorter and more predictable.
- Buses are less overcrowded.
- Elderly and disabled passengers can board safely, because the system never rushes the driver.
- No driver rankings; passenger data is used only in aggregate.

**Expected results (8-week pilot on one route):**
- Excess waiting time falls by 5–15% at real driver behaviour (more in simulation, but we anchor on field figures).
- The share of bunched buses drops below 5% (target).
- The first published regularity figure (CV, excess wait) for a Baku bus route.

**Current stage:**
A working prototype on the real route 61 (AYNA map, OpenStreetMap): simulation, dispatcher console and driver-tablet view. Five scenarios compared: no control, today's terminal dispatch, ideal, Chicago-like and pessimistic.

**Team:**
[Name Surname — role]
[Name Surname — role]
[Name Surname — role]
