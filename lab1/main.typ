#set text(lang: "pl", size: 12pt)
#set par(justify: true)
#set heading(numbering: "1.")
#set grid(column-gutter: 1em)
#set table(align: center + horizon)
#set page(numbering: "1")
#show table: set par(justify: false)
#show table.cell.where(y: 0): strong

#line(length: 100%)

#align(center)[
  #text(size: 2.5em)[*Algorytmy Geometryczne*] \

  #text(size: 1.6em)[*Ćwiczenie 1 - Predykaty geometryczne*] \

  #text(size: 1.2em)[Jakub Własiewicz - Grupa 2]
]

= Opis ćwiczenia

Ćwiczenie polegało na klasyfikacji położenia punktów względem prostej na położone po jej lewej, prawej oraz współliniowe z nią,
a następnie wizualizacji wyników i opisania wniosków.
Do analizy zostały dane następujące zbiory:
- *Zbiór A* : $10^5$ punktów z płaszczyzny $ [-1000, 1000]^2$
- *Zbiór B* : $10^5$ punktów z płaszczyzny $ [-10^14, 10^14]^2$
- *Zbiór C* : $1000$ punktów spełniających równanie okręgu o środku $(0,0)$ i promieniu \ $R = 100$
- *Zbiór D* : $1000$ punktów z przedziału $[-1000,1000]$ znajdujących się na prostej tworzonej przez wektor $accent(a b, arrow)$, gdzie $a = (-1,0)$, $b = (1, 0.1)$

= Dane techniczne
Program został uruchomiony na komputerze z następującymi specyfikacjami:
- *System Operacyjny -* Ubuntu 24.04.3 LTS
- *Architektura Procesora -* x86_64
- *Procesor -* AMD Ryzen 7 7840HS
- *Język i wersja kompilatora -* Rust 1.86.0

Oraz użyte zostały następujące biblioteki:
- *plotters 0.3.7*
- *rand 0.9.2*
- *ndarray 0.16.1*
- *ndarray-linalg 0.17.0*

= Opis teoretyczny
Aby wyznaczyć położenie punktu $c$ względem prostej danej punktami $a$ i $b$, analizujemy wartość wyznacznika macierzy 2x2 lub 3x3 danym następującymi wzorami:
#align(center)[
#grid(
  columns: (auto, auto, auto),
  align: horizon
)[
$ det(a,b,c) = 
    mat(delim: "|", a_x, a_y, 0;
        b_x, b_y, 0;
        c_x, b_y, 0;

    ) 
  $
][oraz][
  #v(1.8pt)
  $ 
    det(a,b,c) = mat(delim: "|", a_x - c_x, a_y - c_y; b_x - c_x, b_y - c_y
    ) 
  $
]
]

$ det(a,b,c) = cases(
  > 0 - "punkt jest po lewej stronie prostej",
  < 0 - "punkt jest po prawej stronie prostej",
  = 0 - "punkt jest współliniowy"
) $

#grid(
  columns: (auto,auto)
)[
  Z tą wiedzą możemy dokonać obliczeń,
  ale ze względu na właściwości arytmetyki
  zmienno-przecinkowej, niezalecane jest
  bezpośrednie przyrówanie wyznacznika do zera.

  Jedną z możliwości jest przyrównanie
  modułu różnicy liczby z zerem do jakiegoś
  małego $epsilon$ :
  $ abs(x - 0.0) <= epsilon $
  Dla wszystkich obliczeń poniżej oprócz zbioru D, przyjmiemy $epsilon = 10^(-16)$.
][
#figure(
  image("prosta.png", width: 60%),
  caption: [Punkt $c$ i prosta wyznaczona przez $a,b$. Punkt $c$ znajduje się po lewej stronie.]
)
]

= Realizacja zadania

Na wszystkich poniższych wizualizacjach do oznaczenia położenia punktów względem prostej zostały użyte następujące kolory:
- #text(blue)[*Niebieski*] - dla punktów współliniowych,
- #text(red)[*Czerwony*] - dla punktów po prawej,
- #text(green)[*Zielony*] - dla punktów po lewej

Oraz następujące oznaczenia dla tablic:
- *f32* - liczby zmienno-przecinkowe 32-bitowe
- *f64* - liczby zmienno-przecinkowe 64-bitowe
- *det_3x3* - wyznacznik macierzy 3x3
- *det_2x2* - wyznacznik macierzy 2x2
- *det_3x3_lib* - wyznacznik macierzy 3x3 z biblioteki _ndarray-linalg_
- *det_2x2_lib* - wyznacznik macierzy 2x2 z biblioteki _ndarray-linalg_

== Zbiór A
#grid(columns: (50%, 50%), rows: (auto))[
#figure(
  caption: [Zbiór A - $10^5$ punktów o współrzędnych z przedziału $[-1000, 1000]$],
  image("plots_typst/1_up.png", width: 90%)
)][
#figure(
  caption: [Zbiór A po pokolorowaniu punktów według kryterium],
  image("plots_typst/1_p.png", width: 90%)
)
]

#figure(
  caption: [Wyniki klasyfikacji zbioru A],
  table(
    table.header(
      [Typ liczb zmienno-przecinkowych],
      [Rodzaj wyznacznika],
      [Liczba punktów po lewej],
      [Liczba punktów po prawej],
      [Liczba punktów współliniowych],
    ),
    columns: 5,
    rows: 8,
    table.cell(rowspan: 4)[f32], [det_2x2], [50276], [49724], [0],
    [det_3x3], [50276], [49724], [0],
    [det_2x2_lib], [50276], [49724], [0],
    [det_3x3_lib], [50276], [49724], [0],
    table.cell(rowspan: 4)[f64], [det_2x2], [50276], [49724], [0],
    [det_3x3], [50276], [49724], [0],
    [det_2x2_lib], [50276], [49724], [0],
    [det_3x3_lib], [50276], [49724], [0],
  )
)
W tym zbiorze, niezależnie od doboru precyzji czy wyznacznika, otrzymujemy te same rezultaty. Zostało to przedstawione w Tabeli 1.

== Zbiór B
#grid(columns: (50%, 50%), rows: (auto))[
#figure(
  caption: [Zbiór B - $10^5$ punktów o współrzędnych z przedziału $[-10^14, 10^14]$],
  image("plots_typst/2_up.png", width: 90%)
)][
#figure(
  caption: [Zbiór B po pokolorowaniu punktów według kryterium dla *f32*, *det_2x2*],
  image("plots_typst/2_p_nofix.png", width: 90%)
)
]

#figure(
  caption: [Wyniki klasyfikacji zbioru B],
  table(
    table.header(
      [Typ liczb zmienno-przecinkowych],
      [Rodzaj wyznacznika],
      [Liczba punktów po lewej],
      [Liczba punktów po prawej],
      [Liczba punktów współliniowych],
    ),
    columns: 5,
    rows: 4,
    table.cell(rowspan: 4)[f32], [det_2x2], [0], [0], [100000],
    [det_3x3], [49871], [50129], [0],
    [det_2x2_lib], [12153], [12182], [75665],
    [det_3x3_lib], [49871], [50129], [0],
    table.cell(rowspan: 4)[f64], [det_2x2], [49868], [50125], [7],
    [det_3x3], [49871], [50129], [0],
    [det_2x2_lib], [49869], [50126], [5],
    [det_3x3_lib], [49871], [50129], [0],
  )
)

Po analizie danych w Tabeli 2, możemy zauważyć, że w przypadku 32-bitowych liczb zmienno-przecinkowych i wyznaczniku 2x2 wszystkie punkty zostały zaklasyfikowane jako współliniowe. Wynika to ze sposobu liczenia tego wyznacznika, utraty precyzji oraz zakresu liczb. Ponieważ liczymy różnicę między liczbami których wszystkie bity mantysy określają cyfry przed przecinkiem, a $0.1$ (druga współrzędna punktu $b$), nie posiadamy dostatecznej precyzji aby otrzymać poprawny wynik,
co powoduje zerowanie wyznacznika.
Jednym z rozwiązań jest przeskalowanie wektora $accent(a b, arrow)$. Przykładowo, możemy pomnożyć przez $10^10$ każdą współrzędną otrzymując: $ a = [-10^10, 0], b = [10^10, 10^9] $
Dzięki temu otrzymujemy te same rezultaty dla każdej kombinacji precyzji i wyznacznika.

#figure(
  image("plots_typst/2_p.png", width: 50%),
  caption: [Pokolorowane punkty zbioru B po przeskalowaniu wektora $accent(a b, arrow)$]
)

#figure(
  caption: [Wyniki klasyfikacji zbioru B po przeskalowaniu wektora $accent(a b, arrow)$],
  table(
    table.header(
      [Typ liczb zmienno-przecinkowych],
      [Rodzaj wyznacznika],
      [Liczba punktów po lewej],
      [Liczba punktów po prawej],
      [Liczba punktów współliniowych],
    ),
    columns: 5,
    rows: 4,
    table.cell(rowspan: 4)[f32], [det_2x2], [49871], [50129], [0],
    [det_3x3], [49871], [50129], [0],
    [det_2x2_lib], [49871], [50129], [0],
    [det_3x3_lib], [49871], [50129], [0],
    table.cell(rowspan: 4)[f64], [det_2x2], [49871], [50129], [0],
    [det_3x3], [49871], [50129], [0],
    [det_2x2_lib], [49871], [50129], [0],
    [det_3x3_lib], [49871], [50129], [0],
  )
)

== Zbiór C
#grid(columns: (50%, 50%), rows: (auto))[
#figure(
  caption: [Zbiór C - 1000 punktów położonych na okręgu o środku $(0,0)$ oraz promieniu \ $R = 100$],
  image("plots_typst/3_up.png", width: 90%)
)][
#figure(
  caption: [Zbiór C po pokolorowaniu punktów według kryterium],
  image("plots_typst/3_p.png", width: 90%)
)
]

#figure(
  caption: [Wyniki klasyfikacji zbioru C],
  table(
    table.header(
      [Typ liczb zmienno-przecinkowych],
      [Rodzaj wyznacznika],
      [Liczba punktów po lewej],
      [Liczba punktów po prawej],
      [Liczba punktów współliniowych],
    ),
    columns: 5,
    rows: 4,
    table.cell(rowspan: 4)[f32], [det_2x2], [504], [496], [0],
    [det_3x3], [504], [496], [0],
    [det_2x2_lib], [504], [496], [0],
    [det_3x3_lib], [504], [496], [0],
    table.cell(rowspan: 4)[f64], [det_2x2], [504], [496], [0],
    [det_3x3], [504], [496], [0],
    [det_2x2_lib], [504], [496], [0],
    [det_3x3_lib], [504], [496], [0],
  )
)
Otrzymujemy te same liczby punktów dla każdej precyzji i wyznacznika. W tym przypadku, aby wygenerować równomiernie punkty na okręgu, losowany był kąt $theta$ z zakresu $[0, 2pi]$, aby skorzystać z równania parametrycznego okręgu:
$ P = (R cos theta, R sin theta) $

== Zbiór D
#figure(
  image("plots_typst/4_up.png", width: 50%),
  caption: [Zbiór D - 1000 punktów z zakresu $[-1000, 1000]$, leżących na prostej wyznaczonej przez wektor $accent(a b, arrow)$]
)

Punkty w tym zbiorze zostały wygenerowane za pomocą prostej $y = 0.05x + 0.05$, którą jednoznacznie wyznaczają punkty $a$ i $b$.

#figure(
  caption: [Wyniki klasyfikacji zbioru D],
  table(
    table.header(
      [$epsilon$],
      [Typ liczb zmienno-przecinkowych],
      [Rodzaj wyznacznika],
      [Liczba punktów po lewej],
      [Liczba punktów po prawej],
      [Liczba punktów współliniowych],
    ),
    columns: 6,
    rows: 4,
    table.cell(rowspan: 8)[$10^(-16)$],
    table.cell(rowspan: 4)[f32], [det_2x2], [167], [170], [663],
    [det_3x3], [77], [326], [597],
    [det_2x2_lib], [170], [210], [620],
    [det_3x3_lib], [68], [217], [715],
    table.cell(rowspan: 4)[f64], [det_2x2], [144], [142], [714],
    [det_3x3], [183], [411], [406],
    [det_2x2_lib], [158], [170], [672],
    [det_3x3_lib], [116], [171], [713],

    table.cell(rowspan: 8)[$10^(-10)$],
    table.cell(rowspan: 4)[f32], [det_2x2], [167], [170], [663],
    [det_3x3], [77], [326], [597],
    [det_2x2_lib], [170], [210], [620],
    [det_3x3_lib], [68], [217], [715],
    table.cell(rowspan: 4)[f64], [det_2x2], [0], [0], [1000],
    [det_3x3], [0], [0], [1000],
    [det_2x2_lib],[0], [0], [1000],
    [det_3x3_lib], [0], [0], [1000],
  )
)
Wizualizacje wszystkich kombinacji dla $epsilon = 10^(-16)$ zbioru D:
#grid(columns: (50%, 50%), rows: (auto,auto,auto,auto))[
#figure(
  caption: [f32 - det_2x2],
  image("plots_typst/4_p_f32_2x2.png", width: 80%)
)][
#figure(
  caption: [f32 - det_3x3],
  image("plots_typst/4_p_f32_3x3.png", width: 80%)
)
][
#figure(
  caption: [f32 - det_2x2_lib],
  image("plots_typst/4_p_f32_2x2lib.png", width: 80%)
)
][
#figure(
  caption: [f32 - det_3x3_lib],
  image("plots_typst/4_p_f32_3x3lib.png", width: 80%)
)
][
#figure(
  caption: [f64 - det_2x2],
  image("plots_typst/4_p_f64_2x2.png", width: 80%)
)
][
#figure(
  caption: [f64 - det_3x3],
  image("plots_typst/4_p_f64_3x3.png", width: 80%)
)
][
#figure(
  caption: [f64 - det_2x2],
  image("plots_typst/4_p_f64_2x2lib.png", width: 80%)
)
][
#figure(
  caption: [f64 - det_3x3_lib],
  image("plots_typst/4_p_f64_3x3lib.png", width: 80%)
)
]

W tym zbiorze wyniki stały się bardzo różnorodne w porównaniu do poprzednich.
Dla $epsilon = 10^(-16)$ wyznacznik 3x3 klasyfikował jako należące do prostej mniej punktów od wszystkich innych
wyznaczników. W przypadku $epsilon = 10^(-10)$, dla f64 jako współliniowe zostały zakwalifikowane wszystkie punkty,
niezależnie od wyboru wyznacznika.

= Wnioski

Z przeprowadzonych obliczeń można wyciągnąć następujące wnioski:

- Dobór precyzji liczb zmienno-przecinkowych ma ogromnie znaczenie, na ogół f64 zapewnia o wiele lepsze wyniki od f32, w szczególności dla dużych zakresów.
- Wybór odpowiedniego sposóbu porównywania liczb zmienno-przecinkowych do naszych potrzeb ma ogromne znaczenie, w tym dobór tolerancji dla zera.
- Implementacja własnego, szczególnego wyznacznika może przynieść wiele korzyści, aniżeli korzystanie z bibliotecznego, generalizowanego wyznacznika $n times n$.
- Warto wykonać testy sprawdzające jak zachowują się dane operacje na liczbach zmienno-przecinkowych przy konkretnych zakresach, jak na przykład w przypadku zbioru B.
- Wizualizacje danych pozwalają czasami na zauważenie wcześniej nieoczywistych, niechcianych anomalii, które po tym jesteśmy w stanie rozwiązać.