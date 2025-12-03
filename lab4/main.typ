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

  #text(size: 1.5em)[*Ćwiczenie 4 - Przecinanie odcinków*] \

  #text(size: 1.2em)[Jakub Własiewicz - Grupa 2 - Poniedziałek 13:00] \
  #text(size: 1.2em)[2025-12-02]
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
- *sortedcontainers*

Do obliczeń została użyta tolerancja dla zera $epsilon = 10^(-9)$, oraz liczby zmienno-przecinkowe o rozmiarze 64 bitów.

= Opis ćwiczenia

Ćwiczenie polegało na implementacji algorytmu zamiatania w celu wyznaczenia przecięć odcinków na płaszczyźnie. Obejmowało to sprawdzenie istnienia przecięcia oraz wyznaczenia wszystkich przecięć.

= Wstęp teoretyczny

== Przecięcie odcinków

#grid(
  columns: 2,
)[
  Dla zbioru odcinków $S = {s_1, s_2, ..., s_n}$ w $bb(R)^2$,
  przecięciem nazywamy taką parę $(s_i,s_j)$, że $i != j$ oraz $s_i inter s_j != emptyset$

  Wprowadzimy następujące założenia:
  - Żaden z odcinków nie jest pionowy,
  - Końcowe współrzędne $x$-owe nie mogą się powtarzać dla każdej pary odcinków,
  - Żadne trzy odcinki nie przecinają się w jednym punkcie,
  - Para odcinków przecina się w co najwyżej jednym punkcie.
][
  #figure(
    caption: [Przykładowy zbiór odcinków w $bb(R)^2$, z zaznaczonymi punktami przecięć],
    image("typst/p1.png")
  )
]

== Algorytm zamiatania

Algorytm zamiatania polega na ustaleniu pewnej hiperpłaszczyzny, w naszym przypadku będzie to prosta, którą będziemy przesuwać po osi $x$. Nazywamy tę prostą "miotłą". Będzie się ona zatrzymywać w 3 różnych interesujących nas zdarzeniach: początek odcinka, koniec odcinka oraz punkt przecięcia. Pozycje te przetrzymujemy w strukturze zdarzeń $Q$, natomiast w strukturze stanu $T$ przechowujemy uporządkowane względem współrzędnej $y$ przecięcia odcinków z miotłą. Sprawdzane względem przecięcia będą tylko odcinki sąsiadujące ze sobą w strukturze stanu, czyli pierwsze odcinki nad oraz pod punktem przecięcia z miotłą.

= Realizacja ćwiczenia

== Wybrane struktury danych

Do realizacji algorytmu zamiatania, jako strukturę stanu $T$ wykorzystano _SortedSet_ z biblioteki _sortedcontainers_. Zapewnia ona łatwe porządkowanie odcinków oraz operacje dodawania, usuwania, wyszukiwania w czasie $O(log n)$. Dzięki temu jesteśmy w stanie efektywnie sprawdzać czy sąsiednie odcinki się przecinają.

W przypadku struktury zdarzeń $Q$, do algorytmu weryfikacji istnienia przecięcia wykorzystana została lista początków i końców odcinków posortowana malejąco aby wykorzystać operację _.pop()_, co pozwala uniknąć przesuwania pozostałych elementów w pamięci. Takie rozwiązanie jest wystarczające ze względu na zakończenie algorytmu w przypadku wykrycia przecięcia, co oznacza brak konieczności dodawania punktów przecięć do struktury zdarzeń.



