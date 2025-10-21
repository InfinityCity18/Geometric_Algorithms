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
  Dzięki temu możemy dokonać obliczeń,
  ale ze względu na właściwości arytmetyki
  zmienno-przecinkowej, nie zalecane jest
  bezpośrednie przyrówanie wyznacznika do zera.

  Jedną z możliwości jest przyrównanie
  modułu różnicy liczby z zerem do jakiegoś
  małego $epsilon$ :
  $ abs(x - 0.0) <= epsilon $
  Dla wszystkich obliczeń poniżej przyjmiemy $epsilon = 10^(-16)$.
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
    rows: 4,
    table.cell(rowspan: 2)[f32], [det_2x2], [50276], [49724], [0],
    [det_3x3], [50276], [49724], [0],
    table.cell(rowspan: 2)[f64], [det_2x2], [50276], [49724], [0],
    [det_2x2], [50276], [49724], [0]
  )
)
W tym zbiorze, niezależnie od doboru precyzji czy wyznacznika, otrzymujemy te same rezultaty. Zostało to przedstawione w Tabeli 1.