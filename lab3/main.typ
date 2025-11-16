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
  #text(size: 1.2em)[2025-11-04]
]

= Dane techniczne
Program został uruchomiony na komputerze z następującymi specyfikacjami:
- *System Operacyjny -* Fedora Linux 43
- *Architektura Procesora -* x86_64
- *Procesor -* AMD Ryzen 7 7840HS
- *Język i wersja kompilatora -* Rust 1.93.0

Oraz użyte zostały następujące biblioteki:
- *config 0.15.18*
- *plotters 0.3.7*
- *rand 0.9.2*
- *serde 1.0.228*

Do obliczeń została użyta tolerancja dla zera $epsilon = 10^(-10)$, oraz liczby zmienno-przecinkowe o rozmiarze 64 bitów.

= Opis ćwiczenia

Ćwiczenie polegało na wyznaczeniu otoczki wypukłej korzystając z algorytmu Grahama i algorytmu Jarvisa, wizualizacji wyników oraz porównania czasów wykonywania dla poniższych zbiorów:

- *Zbiór A* - 100 losowo wygenerowanych punktów o współrzędnych z przedziału $[-100, 100]$
- *Zbiór B* - 100 losowo wygenerowanych punktów leżących na okręgu o środku $(0,0)$ i promieniu $R=10$
- *Zbiór C* - 100 losowo wygenerowanych punktów leżących na bokach prostokąta o wierzchołkach $(-10, 10), (-10,-10), (10,-10), (10,10)$
- *Zbiór D* - zawierający wierzchołki kwadratu $(0, 0), (10, 0), (10, 10), (0, 10)$ oraz po 25 punktów na dwóch bokach kwadratu leżących na osiach i po 20 punktów na przekątnych kwadratu.

= Realizacja zadania

Oba algorytmy najpierw wyznaczały punkt $p_0$ najmniejszy względem współrzędnej $y$ oraz najmniejszy względem $x$ jeśli poprzednie współrzędne były równe.

W algorytmie Grahama punkty zostały posortowane za pomocą funkcji bibliotecznej ze względu na kąt tworzący z poziomą osią oraz prostą od $p_0$ do rozpatrywanego punktu. Do tego celu wykorzystano wyznacznik macierzy 3x3. Z punktów współliniowych zostawiamy tylko ten z największą odległością od $p_0$.

W algorytmie Jarvisa szukamy punktów których kąt w odniesieniu do ostatniej krawędzi jest najmniejszy. Do tego również został wykorzystany wyznacznik 3x3. W przypadku współliniowości, rozpatrujemy tylko dalszy punkt.

Złożoność czasowa algorytmu Grahama to $O(n log n)$, a Jarvisa $O(n k)$, gdzie $n$ - liczba punktów w zbiorze, $k$ - liczba punktów otoczki.

Na wszystkich poniższych rysunkach następujące kolory oznaczają:
- #text(black)[*Czarny*] - punkty zbioru,
- #text(green)[*Zielony*] - punkty zbioru zaliczone jako należące do otoczki
- #text(red)[*Czerwony*] - boki otoczki,
