#set text(lang: "pl", size: 12pt)
#set par(justify: true)
#set heading(numbering: "1.")
#set grid(column-gutter: 1em)
#set table(align: center + horizon)
#set page(numbering: "1")
#show table: set par(justify: false)
#show table.cell.where(y: 0): strong

#let unit(x) = $space  upright(#x) $

#line(length: 100%)

#align(center)[
  #text(size: 2.5em)[*Algorytmy Geometryczne*] \

  #text(size: 1.5em)[*Ćwiczenie 3 - Triangulacja wielokątów monotonicznych*] \

  #text(size: 1.2em)[Jakub Własiewicz - Grupa 2 - Poniedziałek 13:00] \
  #text(size: 1.2em)[2025-11-18]
]

= Dane techniczne
Program został uruchomiony na komputerze z następującymi specyfikacjami:
- *System Operacyjny -* Fedora Linux 43
- *Architektura Procesora -* x86_64
- *Procesor -* AMD Ryzen 7 7840HS
- *Język -* Python 3.14.0

Ćwiczenie realizowane było w środowisku _Jupiter_,
do wizualizacji zostało użyte narzędzie stworzone przez koło naukowe _BIT_ 
oraz następujące biblioteki:
- *matplotlib*
- *numpy*
- *pandas*

Do obliczeń została użyta tolerancja dla zera $epsilon = 10^(-16)$, oraz liczby zmienno-przecinkowe o rozmiarze 64 bitów.

= Opis ćwiczenia

Ćwiczenie polegało na sprawdzeniu czy podany wielokąt jest $y$-monotoniczny, klasyfikacji wierzchołków w dowolnym wielokącie oraz triangulacji wielokąta $y$-monotonicznego, w tym wizualizacji, analizy i stwierdzenia wniosków.

#pagebreak(weak: true)

= Wstęp teoretyczny

== Wielokąt $y$-monotoniczny <3.1>

#grid(
  columns: (50%, auto),
  align: (horizon)
)[
  #figure(
    caption: [Przykładowy wielokąt $y$-monotoniczny],
    image("typst/przyklad_mono.png", width: 100%)
  )
][
  Wielokąt nazywamy $y$-monotonicznym, jeżeli możemy podzielić jego punkty na dwa takie łańcuchy, że współrzędne $y$-owe punktów są niemalejące w jednym z nich oraz nierosnące w drugim.

  Zaimplementowany algorytm sprawdzający $y$-monotoniczność wielokąta polega na znalezieniu najniższego punktu względem osi $y$, a następnie ustawieniu dwóch indeksów, które poruszają się odpowiednio wzdłuż lewego i prawego łańcucha. Idąc kolejnymi punktami, jeżeli jeden z nich znajdzie się w punkcie o mniejszej $y$-owej współrzędnej niż poprzedni badany punkt, zapamiętujemy, że jeden z nich "zawrócił". 
  Jeżeli wydarzy się to po raz drugi, to wielokąt nie jest $y$-monotoniczny.
]

== Klasyfikacja wierzchołków

#grid(
  columns: (50%, auto),
  align: (horizon)
)[
  Możemy zaklasyfikować wierzchołki wielokąta następująco:
  - *#text([Początkowy], green) -* obaj jego sąsiedzi leżą poniżej i kąt wewnętrzny $< pi$
  - *#text([Końcowy], red) -* obaj jego sąsiedzi leżą powyżej i kąt wewnętrzny $< pi$
  - *#text([Łączący], blue) -* obaj jego sąsiedzi leżą powyżej i kąt wewnętrzny $> pi$
  - *#text([Dzielący], aqua) -* obaj jego sąsiedzi leżą poniżej i kąt wewnętrzny $> pi$
  - *#text([Prawidłowy], rgb("#552c03")) -* jeden sąsiad powyżej, drugi poniżej
][
  #figure(
    caption: [Przykładowa klasyfikacja wierzchołków wielokąta],
    image("typst/przyklad_wierz.png", width: 100%)
  )
]

== Triangulacja wielokąta monotonicznego

=== Definicja triangulacji

#grid(
  columns: 2,
)[
  #figure(
    caption: [Przykładowa triangulacja wielokąta],
    image("typst/przyklad_trian.png")
  )
][#move(dy: 0.5em)[Triangulacją wielokąta nazywamy zbiór tworzący ten wielokąt z trójkątów z niepustymi oraz rozłącznymi wnętrzami, gdzie ich przecięcie jest puste lub zredukowane do punktu albo do krawędzi.
Na ogół, dla danego wielokąta istnieje wiele różnych triangulacji. 
Zajmiemy się triangulacją wyłącznie wielokątów monotonicznych, jednym ze sposobów triangulacji niemonotonicznych wielokątów jest podzielenie go na mniejsze, monotoniczne wielokąty.
]]

=== Algorytm triangulacji <3.3.2>

Pierwszym krokiem jest uporządkowanie punktów wielokątu względem monotoniczności, w tym określenie należności punktu do lewego lub prawego łańcucha. 
Następnie wkładamy dwa pierwsze punkty na stos i wywołujemy pętlę iterującą po naszej uporządkowanej liście punktów:

Jeżeli wybrany punkt nie należy do tego samego łańcucha co punkt na szczycie stosu to należy go połączyć ze wszystkimi wierzchołkami na stosie, a następnie zostawić na stosie dany punkt oraz jego szczyt.

W przeciwnym wypadku, analizujemy kolejne trójkąty które tworzy wybrany punkt ze szczytem stosu oraz następującymi jego elementami,
jeżeli trójkąt należy do wielokąta to dodajemy przekątną i usuwamy "zewnętrzny" punkt. Innymi słowy, usuwamy tyle ile możemy "zewnętrznych" trójkątów, aby uzyskać mniejszy wielokąt.

= Realizacja obliczeń

Zaimplementowana została weryfikacja $y$-monotoniczności wielokąta. @3.1 opisuje wybrany algorytm. Do klasyfikacji wierzchołków została użyta funkcja `orient` licząca wyznacznik 3x3. @3.3.2 ukazuje postępowanie użytego algorytmu triangulacji. Wielokąty są przechowywane jako lista wierzchołków, gdzie $i$-ty tworzy bok z $(i+1)$-tym, oraz pierwszy i ostatni.

Użytkownik jest w stanie wprowadzić wielokąt za pomocą myszki wybierając kolejne punkty w kierunku przeciwnym do wskazówek zegara lub wczytać go z pliku. Przewidziane jest też zapisanie wielokątu do pliku oraz jego triangulacji w postaci boków i przekątnych wielokąta określanych przez listę par indeksów oznaczających numer wierzchołka. Istnieje możliwość wyświetlenia animacji triangulacji do pliku `.gif`

Do analizy zostały przyjęte następujące wielokąty:

#pagebreak(weak: true)

- *Choinka* - wielokąt z kątami ostrymi lub rozwartymi, na przemian łańcuchy
- *Grot* - wielokąt w którym podążamy tylko prawym łańcuchem z początku
- *Grzebień* - wielokąt z kolcami na prawym łańcuchu
- *Gwiazda* - niesymetryczny wielokąt przypominający gwiazdę
- *Okrąg* - wielokąt zbliżony kształtem do okręgu
- *Sześciokąt* - wielokąt będący sześciokątem
- *Krab* - niemonotoniczny wielokąt

#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Wielokąt _choinka_],
    image("typst/choinka.png")
  )
  ][
  #figure(
    caption: [Wielokąt _grot_],
    image("typst/grot.png")
  )
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Wielokąt _grzebień_],
    image("typst/grzebien.png")
  )
  ][
  #figure(
    caption: [Wielokąt _gwiazda_],
    image("typst/gwiazda.png")
  )
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Wielokąt _okrąg_],
    image("typst/okrag.png")
  )
  ][
  #figure(
    caption: [Wielokąt _sześciokąt_],
    image("typst/szesciokat.png")
  )
]]
#figure(
  caption: [Wielokąt _krab_],
  image("typst/krab.png",width: 50%)
)


= Analiza obliczeń

== Sprawdzenie $y$-monotoniczności

#figure(
  caption: [Wyniki funkcji `is_y_monotonic` dla danych wielokątów],
  table(
    columns: 2,
    table.header([*Nazwa wielokąta*], [*Wynik funkcji `is_y_monotonic`*]),
    [Choinka], [`true`],
    [Grot], [`true`],
    [Grzbień], [`true`],
    [Gwiazda], [`true`],
    [Okrąg], [`true`],
    [Sześciokąt], [`true`],
    [Krab], [`false`],
  )
)

Zatem funkcja poprawnie weryfikuje monotoniczność wielokątów, ponieważ dla wielokąta _krab_ oczekiwaliśmy wartości `false`, a dla reszty `true`.

== Klasyfikacja wierzchołków

#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Klasyfikacja wierzchołków wielokątu _choinka_],
    image("typst/choinka_color.png")
  )
  ][
  #figure(
    caption: [Klasyfikacja wierzchołków wielokątu _grot_],
    image("typst/grot_color.png")
  )
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Klasyfikacja wierzchołków wielokątu _grzebień_],
    image("typst/grzebien_color.png")
  )
  ][
  #figure(
    caption: [Klasyfikacja wierzchołków wielokątu _gwiazda_],
    image("typst/gwiazda_color.png")
  )
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure( 
    caption: [Klasyfikacja wierzchołków wielokątu _okrąg_],
    image("typst/okrag_color.png")
  ) <r15>
  ][
  #figure(
    caption: [Klasyfikacja wierzchołków wielokątu _sześciokąt_],
    image("typst/szesciokat_color.png")
  )
]]
#figure(
  caption: [Klasyfikacja wierzchołków wielokątu _krab_],
  image("typst/krab_color.png",width: 50%)
)

Możemy zauważyć, że wielokąty $y$-monotoniczne nie posiadają żadnych wierzchołków łączących i dzielących. @r15 przedstawia wielokąt _okrąg_ gdzie nie ma żadnych wierzchołków początkowych. Wynika to z dobranej tolerancji dla zera. Nie ma to wpływu na jego monotoniczność.

== Triangulacja

#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Triangulacja wielokąta _choinka_],
    image("typst/choinka_tri.png")
  ) <r18>
  ][
  #figure(
    caption: [Triangulacja wielokąta _grot_],
    image("typst/grot_tri.png")
  )
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Triangulacja wielokąta _grzebień_],
    image("typst/grzebien_tri.png")
  ) <r20>
  ][
  #figure(
    caption: [Triangulacja wielokąta _gwiazda_],
    image("typst/gwiazda_tri.png")
  ) <r21>
]]
#grid(rows: 4, align: center + horizon)[
  #grid(columns: 2)[
  #figure(
    caption: [Triangulacja wielokąta _okrąg_],
    image("typst/okrag_tri.png")
  )
  ][
  #figure(
    caption: [Triangulacja wielokąta _sześciokąt_],
    image("typst/szesciokat_tri.png")
  )
]]
#figure(
  caption: [Triangulacja wielokąta _krab_],
  image("typst/krab_tri.png",width: 50%)
) <r24>

Wykresy triangulacji wielokątów powyżej potwierdzają poprawność zaimplementowanego algorytmu. Dodatkowo został on uruchomiony na wielokącie _krab_, który nie spełnia założenia $y$-monotoniczności.

@r18 pokazuje jak algorytm poradził sobie w przypadku punktów z lewego i prawego łańcucha na zmianę ustawionych względem monotoniczności oraz wielu kątów ostrych i rozwartych.

Wielokąt _grot_ ukazuje przypadek gdzie stos stanie się najdłuższy ze względu na ustawienie prawie wszystkich punktów na prawym łańcuchu oraz wspólnej niemożliwości stworzenia trójkątów z wierzchołków na stosie, ponieważ nie zawierałyby się one we wnętrzu wielokąta

@r20 przedstawia możliwość powstania trójkąta w triangulacji którego bokami są wyłącznie przekątne.

@r21 z pozoru przedstawia błędną triangulację, po przybliżeniu wykresu można zauważyć poprawność triangulacji, ukazuje to rysunek 25.

#figure(
  caption: [Przybliżony wykres triangulacji wielokąta _gwiazda_],
  image("typst/gwiazda_zoom.png", width: 70%)
)

Dla wielokątów _okrąg_ i _sześciokąt_ triangulacja została przeprowadzona pomyślnie.

@r24 przedstawia nieudaną triangulację wielokąta niemonotonicznego.

= Wnioski

Zaimplementowany algorytm weryfikowania $y$-monotoniczności poprawnie zadziałał dla wszystkich wielokątów.
Klasyfikacja wierzchołków przebiegła pomyślnie dla każdego przypadku.
Algorytm triangulacji poradził sobie ze wszystkimi zadanymi wielokątami monotonicznymi. Ze sprawozdaniem zostało dołączone archiwum z animacjami triangulacji w formacie `.gif`







