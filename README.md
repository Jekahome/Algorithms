
## Зачем изучать алгоритмы?
- Алгоритмы - это про способ мыслить о программах
- Вы сможете писать оптимизированный и масштабируемый код - получив знания о различных структурах данных и алгоритмах, вы можете определить, какую выбрать структуру данных и алгоритм в различных условиях.
- Эффективное использование времени и памяти. Знание структур данных и алгоритмов поможет вам писать код, который работает быстрее и требует меньшего объема памяти.
 
## [Книги про алгоритмы](https://elbrusboot.camp/blog/top-11-knigh-po-alghoritmam/)
- Введение в алгоритмы, *Томас Х. Кормен* - это одна из лучших книг по алгоритмам, в которой подробно рассматривается широкий спектр алгоритмов.
- Алгоритмы *Роберта Седжвика* - это ведущий учебник по алгоритмам, широко используемый в колледжах и университетах.
- Искусство программирования, *Дональд Э. Кнут* - эта книга считается лучшей, если вы знакомы с предметом и ищете более глубокое понимание.

## Алгоритмические метод решения задач:

- **Brute-Force** - перебирайте все варианты в лоб.

    В чем разница между алгоритмом `Backtracking Algorithm` и алгоритмом `Brute-force`?

    В связи с тем, что алгоритм поиска `Backtracking` принимает для принятия решения все возможные результаты, с этой точки зрения он подобен алгоритму `Brute-force`. Разница состоит в том, что иногда `Backtracking` может обнаружить, что полный перебор не нужен, и, следовательно, может работать намного лучше.

- **Divide and Conquer** - разбивайте задачу на подзадачи и комбинируйте их (используют в алгоримах `Merge sort` и `Binary Search`). Разделяй и властвуй — это парадигма разработки алгоритма, основанная на многоветвящейся рекурсии. Алгоритм `Divide and Conquer` разбивает проблему на подзадачи того же или родственного типа, пока они не станут достаточно простыми, чтобы их можно было решить напрямую.

    Алгоритмы «разделяй и властвуй» представляют собой парадигму решения задач, включающих несколько основных шагов. Сначала мы делим проблему на более мелкие части и работаем над решением каждой из них независимо. После того, как мы решили все части, мы берем все полученные более мелкие решения и объединяем их в одно интегрированное комплексное решение.

    Этот процесс может выполняться рекурсивно; то есть каждая «подзадача» сама по себе при необходимости может быть разделена на еще более мелкие части. Это рекурсивное деление задачи выполняется до тех пор, пока каждая отдельная проблема не станет достаточно маленькой, чтобы ее решение стало относительно тривиальным.

    Некоторыми распространенными примерами задач, которые хорошо подходят для этого подхода, являются `Binary Search`, алгоритмы сортировки (например, `Merge sort`, `Quicksort`), оптимизация сложных в вычислительном отношении математических операций (возведение в степень, БПФ, алгоритм Штрассена) и другие.

    Ниже приведены некоторые стандартные алгоритмы, соответствующие алгоритму `Divide and Conquer`.  

    - `Quicksort` — это алгоритм сортировки. Алгоритм выбирает опорный элемент и переупорядочивает элементы массива так, чтобы все элементы, меньшие, чем выбранный опорный элемент, перемещались в левую сторону опорного элемента, а все элементы большего размера перемещались в правую сторону. Наконец, алгоритм рекурсивно сортирует подмассивы слева и справа от опорного элемента.
    - `Merge sort` также является алгоритмом сортировки. Алгоритм делит массив на две половины, рекурсивно сортирует их и, наконец, объединяет две отсортированные половины.
    - Ближайшая пара точек. Задача состоит в том, чтобы найти ближайшую пару точек в наборе точек в плоскости `xy`. Проблему можно решить за время `O(n^2)`, вычислив расстояния до каждой пары точек и сравнив расстояния, чтобы найти минимум. Алгоритм «разделяй и властвуй» решает проблему за время `O(N log N)`.
    - Алгоритм Штрассена — это эффективный алгоритм умножения двух матриц. Простой метод умножения двух матриц требует трех вложенных циклов и равен `O(n^3)`. Алгоритм Штрассена умножает две матрицы за время `O(n^2,8974)`.
    - Алгоритм быстрого преобразования Фурье (БПФ) Кули – Тьюки является наиболее распространенным алгоритмом БПФ. Это алгоритм «разделяй и властвуй», который работает за время `O(N log N)`.
    - Алгоритм Карацубы для быстрого умножения выполняет умножение двух n - значных чисел  

    ```
    DAC(a, i, j) {
        if(small(a, i, j))
            return(Solution(a, i, j))
        else 
            mid = divide(a, i, j)   // f1(n)
            b = DAC(a, i, mid)      // T(n/2)
            c = DAC(a, mid+1, j)    // T(n/2)
            d = combine(b, c)       // f2(n)
        return(d)
    }
    ```

    [geeksforgeeks](https://www.geeksforgeeks.org/complete-roadmap-to-learn-dsa-from-scratch/)

- **Dynamic Programming** - Кешируйте промежуточные результаты для их повторного использования. `DP = Recursion + some memory`

    Подход `Memoization` (`Сверху Вниз`) - кеширования результатов повторяющихся расчетов в таблицу и их использования по мере хода алгоритма.
    
    Подход `Tabulation` (`Снизу Вверх`) - кеширование всех результатов подзадач в таблицу и после в нужной последовательности сбор результатов. Пример, задача о максимальной вместимости рюкзака, для перебора всех возможных вариантов `O(2^N)`, быстрее решать подзадачи `O(N^2)`.

    Подход `Memoization`. Функции могут использовать объекты для запоминания результатов предыдущих операций, что позволяет избежать ненужной работы. Такая оптимизация называется мемоизацией .
    Мемоизация Фибоначчи. Алгоритм вычисления чисел Фибоначчи. 
    Его дерево рекурсивных вызовов показывает, что fib(3) вычисляется многократно. 
    Мы можем это исправить, сохраняя результаты по мере их вычисления и делая новые вызовы fib только для тех вычислений, результатов которых еще нет в памяти многократного использования промежуточных результатов называется мемоизацией.

    [Как классифицировать проблему как проблему динамического программирования?](https://www.geeksforgeeks.org/solve-dynamic-programming-problem/?ref=lbp)

    Как правило, все проблемы, требующие максимизации или минимизации определенных величин, или задачи подсчета, требующие подсчета композиций при определенных условиях, или определенные проблемы вероятности, могут быть решены с помощью динамического программирования.
    
    Все задачи динамического программирования удовлетворяют свойству перекрывающихся подзадач, а большинство классических задач динамического программирования также удовлетворяют свойству оптимальной подструктуры. Как только мы обнаружим эти свойства в данной задаче, будьте уверены, что ее можно решить с помощью динамического программирования.

    [Dynamic Programming topcoder](https://www.topcoder.com/thrive/articles/Dynamic%20Programming:%20From%20Novice%20to%20Advanced) - Кешируйте промежуточные результаты для их повторного использования. 

    [Dynamic Programming brestprog](https://brestprog.by/topics/dp/)

    [Dynamic Programming brestprog](https://brestprog.by/topics/bitmasks/)

    [Dynamic Programming topcoder](https://www.topcoder.com/thrive/tracks?tax=Dynamic%20Programming&track=Competitive%20Programming)

    [Динамическое программирование](https://ru.algorithmica.org/cs/general-dynamic/)

- **Greedy Algorithm** - пытайтесь взять что-то выгодное в первую очередь (жадный алгоритм). Жадные алгоритмы используются для решения задач оптимизации, делая локально оптимальный выбор на каждом шаге. В этих алгоритмах решения принимаются на основе информации, доступной в текущий момент, без учета последствий этих решений в будущем. Основная идея состоит в том, чтобы на каждом этапе выбрать наилучший возможный вариант, ведущий к решению, которое не всегда может быть самым оптимальным, но часто достаточно хорошим для многих проблем.

    Например: 
    - Кодирование Хаффмана — это алгоритм сжатия данных, который формулирует основную идею сжатия файлов.
    - Проблема размена монет. Жадный алгоритм можно использовать для сдачи заданной суммы с минимальным количеством монет, всегда выбирая монету с наибольшей стоимостью, которая меньше оставшейся суммы, подлежащей обмену.


    [Greedy Algorithm](https://www.geeksforgeeks.org/greedy-algorithms/)

    [Introduction to Greedy Algorithm](https://www.geeksforgeeks.org/introduction-to-greedy-algorithm-data-structures-and-algorithm-tutorials/)

    [Greedy Algorithm](https://www.programiz.com/dsa/greedy-algorithm)

- **Blacktracking** - (поиск с возвратом) оптимизация `Brute-Force` отменяя отдельные затратные операции еще до начала их выполнения. Т.е. отбросить заведомо неверный путь, недопустив затрат на его расчет еще до начала самого расчета. Это общий метод нахождения решений задачи, в которой требуется полный перебор всех возможных вариантов в некотором множестве М. 
Это сама по себе рекурсия, но здесь есть некоторые дополнительные условия, которые делают ее более эффективной.

    Проблемы, которые обычно решаются с использованием метода обратного отслеживания, имеют следующее общее свойство. Эти проблемы можно решить, только перепробовав все возможные конфигурации, причем каждая конфигурация проверяется только один раз. Наивное решение этих проблем — попробовать все конфигурации и вывести конфигурацию, соответствующую заданным ограничениям задачи. Обратное отслеживание работает постепенно и представляет собой оптимизацию по сравнению с решением `Naive`, при котором генерируются и опробуются все возможные конфигурации.

    Как упоминалось ранее, алгоритм `Blacktracking` является производным от алгоритма рекурсии и имеет возможность вернуться в исходное состояние в случае сбоя рекурсивного решения, т. е. в случае сбоя решения программа возвращается к моменту, когда оно потерпело неудачу, и основывается на другом решении. По сути, он пробует все возможные решения и находит правильное.

    Обход с возвратом — это алгоритмический метод решения проблем рекурсивно, пытаясь построить решение постепенно, по одному фрагменту за раз, удаляя те решения, которые не удовлетворяют ограничениям проблемы в любой момент времени (под временем здесь подразумевается время, прошедшее до достижения любого уровня дерева поиска).

    Некоторые стандартные проблемы:
    - Задача решения судоку
    - Крыса в лабиринте
    - Сумма комбинации, Сумма комбинации II, Сумма комбинации III
    - Проблема с королевой N
    - Проблема N-ферзей

- **Local Search** - пытайтесь выбрать решение которое не хуже предыдущего (градиентный спуск/подьем).

    Примером [локального поиска](https://www.simplilearn.com/local-search-algorithms-in-ai-article#:~:text=Local%20search%20algorithms%20focus%20on%20finding%20solutions%20within%20a%20limited,explore%20the%20entire%20solution%20space.) является алгоритм «Восхождение на холм». Он начинается с первоначального решения и итеративно вносит небольшие изменения для улучшения текущего решения с целью найти локально оптимальное решение в ограниченной части пространства решений.

- **Transform and Conquer** - преобразование данных для лучшего их использования или преобразование самой задачи (индекс в базах данных, разместить данные особым способом для быстрого поиска. Инвертированные индекс и Column-oriented DB MS).

    [Способы применения:](https://www.geeksforgeeks.org/transform-and-conquer-technique/)
   
    1. Упрощение экземпляра: Это можно сделать, уменьшив размер проблемы, разбив ее на более мелкие части или изменив структуру проблемы. Этот метод можно использовать для упрощения задач, которые слишком велики для решения традиционными методами.
   
    2. Уменьшение проблемы: Идея сокращения проблем состоит в том, чтобы превратить данную проблему в другую, которую легче решить. Это можно сделать, преобразовав проблему в другую форму или используя эвристику для поиска решения.

    3. Изменение представления: Основная идея этого метода — изменить представление данных, чтобы их можно было упростить. Это можно сделать путем изменения входных данных или выходных данных.

 
- **Randomized algorithm** - (рандомизированный алгоритм) используют случайность для решения проблемы. Это может быть полезно для решения проблем, которые не могут быть решены детерминистически, или для повышения средней сложности задачи. Рандомизированная быстрая сортировка: вариант алгоритма быстрой сортировки, в котором точка опоры выбирается случайным образом.

    [Randomized algorithm](https://www.geeksforgeeks.org/randomized-algorithms/)

## Sorting Algorithms

Сортировка является фундаментальной операцией в компьютерных науках, и для неё существует несколько эффективных алгоритмов, таких как `Quicksort`, `Merge sort` и `Heapsort`.

[Radix Sort](https://thecode.media/radix/) — сортировка по основанию системы счисления.

Главное коротко: алгоритм поразрядной сортировки гениален в том, что сортирует не числа целиком, а значения разрядов. Получается, что он как бы разбирается с числами на уровне единиц, десятков, сотен и т. д. и только потом он делает общую сортировку. Это позволяет ему не бегать по всем сравниваемым числам и не делать миллион сравнений. Отсюда и экономия времени.

[Временная сложность Radix Sort](https://www.geeksforgeeks.org/radix-sort/) `O(d * (n + b))` , где d — количество цифр, n — количество элементов, а b — основа используемой системы счисления.

В практических реализациях поразрядная сортировка `Radix Sort` часто работает быстрее, чем другие алгоритмы сортировки на основе сравнения, такие как `Quicksort` или `Merge sort`, для больших наборов данных, особенно когда ключи содержат много цифр. Однако его временная сложность растет линейно с увеличением количества цифр, поэтому он не так эффективен для небольших наборов данных.

Вспомогательное пространство `Radix Sort` `O(n + b)`, где n — количество элементов, а b — основание системы счисления. 


[Sorting Algorithms](https://www.geeksforgeeks.org/sorting-algorithms/)

[Counting sort](https://www.geeksforgeeks.org/counting-sort/)

![Sort algorithms.](https://github.com/Jekahome/Algorithms/blob/main/_img/algo_sort.jpg "Sort algorithms.")

 
## Searching Algorithms

Поиск элемента в большом наборе данных — распространенная задача, и для неё существует несколько эффективных алгоритмов, таких как бинарный поиск и хеш-таблицы.

Наиболее распространенные алгоритмы поиска:

- [`Linear Search`](https://www.geeksforgeeks.org/linear-search/) (Линейный поиск) `O(n)` - проверяем элемент итеративно от одного конца к другому.
- [`Binary Search`](https://www.geeksforgeeks.org/binary-search/) (Двоичный поиск) `O(log n)` - разбиваем структуру данных на две равные части и пытаемся решить, в какой половине нам нужно найти элемент. 
- [`Ternary Search`](https://www.geeksforgeeks.org/ternary-search/) (Тернарный поиск) `O(2 * log3n)` - массив делится на три части, и на основе значений в позициях разделения мы определяем сегмент, в котором нам нужно найти нужный элемент.
- [`Jump Search`](https://www.geeksforgeeks.org/jump-search/) между `O(n)` и двоичным поиском `O(Log n)` - это алгоритм поиска в отсортированных массивах. Основная идея состоит в том, чтобы проверять меньше элементов (по сравнению с линейным поиском ), переходя вперед на фиксированные шаги или пропуская некоторые элементы вместо поиска по всем элементам. Если мы сравним его с линейным и бинарным поиском, то окажется, что он лучше, чем линейный поиск, но не лучше, чем бинарный поиск.
- [`Interpolation Search`](https://www.geeksforgeeks.org/interpolation-search/) `O(log 2 (log 2 n))` для среднего случая и `O(n)` для наихудшего случая - это улучшение по сравнению с двоичным поиском для случаев, когда значения в отсортированном массиве распределены равномерно. Интерполяция создает новые точки данных в диапазоне дискретного набора известных точек данных. Двоичный поиск всегда обращается к среднему элементу для проверки. С другой стороны, интерполяционный поиск может осуществляться в разных местах в зависимости от значения искомого ключа. Например, если значение ключа ближе к последнему элементу, интерполяционный поиск, скорее всего, начнет поиск в направлении конечной стороны.
- [`Exponential Search`](https://www.geeksforgeeks.org/exponential-search/) `O(log n)` - Он работает лучше, чем двоичный поиск, для ограниченных массивов, а также когда искомый элемент находится ближе к первому элементу.

`Binary Search` и `Exponential Search` это алгоритмы поиска значения в отсортированном (подготовленном) наборе данных, но если характер использования подразумевает постоянное изменение набора данных, то эти алгоритмы будут иметь значительно меньшую производительность чем [`Red Black Tree`](https://github.com/Jekahome/Data-Structures/tree/main/src/red_black_tree)

![Searching Algorithms.](https://github.com/Jekahome/Algorithms/blob/main/_img/Searching-Algorithms.png "Searching Algorithms.")

[geeksforgeeks](https://www.geeksforgeeks.org/complete-roadmap-to-learn-dsa-from-scratch/)

 
### Фильтр Блума и HyperLogLog

Вероятностные структыры данных. Они дают ответ,который может оказаться ложным, но с большой вероятностью является правильным. 
Фильтры Блума очень удобны тогда, когда не нужно хранить точный ответ. Пример, что бы уменьшить количество проверок наличия ключа в базе можно получить вероятный ответ наличия или точно отрицательный используя Фильтр Блума

### Линейное программирование

Линейное программирование используется для максимизации некоторой характеристики при заданных ограничениях. 
Предположим, ваша компания выпускает два продукта: рубашки и сумки. На рубашку требуется 1 м 5 пуговиц. 
На изготовление сумки необходимо 2 м ткани и 2 пуговицы. У вас есть 11 м ткани и 20 пуговиц. 
Рубашка приносит прибыль $2, а сумка - $3. Сколько рубашек и сумок следует изготовить для получения ткани и максимальной прибыли?

## Bit Manipulation / Bit Masking:

Чтобы две цифры от 0-9 можно было закодировать в одном байте

```
AND (&)
OR (|)
XOR (^)
NOT (~)
Left Shift (<<)
Right Shift(>>)
```

[Data Compression: Bit-Packing 101](https://kinematicsoup.com/news/2016/9/6/data-compression-bit-packing-101)


## [Комбинаторика](https://practicum.yandex.ru/blog/perestanovki-razmescheniya-sochetaniya-v-analize-dannyh/)

Комбинато́рика — раздел математики, посвящённый решению задач, связанных с выбором и расположением элементов некоторого (чаще всего конечного) множества в соответствии с заданными правилами. 

## Задача NP-полная (NP-complete problem)

Тип задач, принадлежащих классу `NP` (non-deterministic polynomial – «недетерминированные с полиномиальным временем» `O(N^2)`, `O(N^3)`), для которых отсутствуют быстрые алгоритмы решения. Время работы алгоритмов решения таких задач существенно (обычно, экспоненциально) возрастает с увеличением объема входных данных.

Буква `P` в названии означает полиномиальную сложность алгоритма  `A_0X^n + ... + A_n`
Буква `N` - оценка сложности соответствует выполнению на недетерминированной машине Тьюринга, иначе — обычной.

Однако, если предоставить алгоритму некоторые дополнительные сведения, то временные затраты могут быть существенно снижены. При этом, если будет найден быстрый алгоритм для какой-либо из NP-полных задач, то для любой другой задачи из класса NP можно будет найти соответствующее решение.

К классу NP-полных относятся задача о коммивояжере, о вершинном покрытии и покрытии множеств, восстановление поврежденных файлов, оптимизация маршрутов, сложные вычисления в биоинформатике.

Криптография открытых ключей основывается на предположении, что `NP ≠ P`. Если найдется способ решать задачи этого класса за полиномиальное время, то многие методы защиты больше не будут иметь смысла.


## Асимптотическая сложность алгоритмов

Двумя фундаментальными понятиями в этой области являются временная сложность и пространственная сложность

`Big O` - описывает наихудший сценарий того, сколько времени потребуется алгоритму для решения проблемы по мере роста входных данных.

`Big O Space` - сколько памяти (пространства для хранения) требуется алгоритму для работы с входными данными.

Сложность алгоритма - это то, как будет расти потребление ресурсов с увеличением N количества елементов до бесконечности. Наивная реализация алгоритма/задачи которая превышает заданный лимит времени выполнения требует другого решения с учетом оптимизации количества операций и выделенных ресурсов. Если ваше решение не укладывается в ограничение по памяти, оно скорее всего не уложится и в ограничение по времени.

Количество операций для n = 10_000 елементов:

-  `O(1)`, константная - 1 операция
-  `O(log n)`, логарифмическая (как в бинарном поиске) - ~13 операций
-  `O(n)`, линейная - 10_000 операций
-  `O(n^2)`, квадратичная - 100_000_000 операций
-  `O(n^3)`, кубическая (как в тройном влоденном цикле) - 1_000_000_000_000 операций
-  `O(n!)`, факториал 10 000 000 000 000 000 000 000 000 ... операций


![Comparison of algorithms.](https://github.com/Jekahome/Algorithms/blob/main/_img/algo.jpg "Comparison of algorithms.")

## Асимптотическая сложность ф-ций в нотации Big-O:

1. константная `O(1)` 
2. логарифмическая `O(log N)`, `O(log^2 N)`
3. корень из N `O(sqrt N)`
4. линейная `O(N)` 
5. линейная `O(N+M)` 
6. линеарифмическая/linearithmic `O(N*log N)`, `O(N*log^2 N)` или `O(N*M)`,`O(N*sqrt M)`
7. полиномиальная квадратичная `O(N^2)`,`O(N^2*log N)`  или кубическая `O(N^3)` 
8. экспоненциальная `O(2^N)`
9. факториал `O(n!)`
 
```rust
/* 
Временная сложность алгоритма - константная O(1)
Это лучшее. 
Алгоритм всегда занимает одинаковое количество времени, независимо от объема данных. 
Пример: поиск элемента массива по его индексу.
*/
fn algo_1(v: &[i32], index: usize) -> Option<i32>{
    if index < v.len(){
        return Some(v[index]);
    }
    None    
} 
// или
let x = &[1, 2, 4];
unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}

// или
vec.push(), vec.pop()

``` 

```rust
/* 
Временная сложность алгоритма - логарифмическая O(log N)
(`log N` это `log _2 N` в какой степени должна быть двойка чтобы получилось N, 
это и будет количество операций)
каждая итерация сокращает вдвое количество элементов/значений
Перевод числа в двоичное представление
TODO: Логарифм по основанию `a` от аргумента `x` — это степень, 
в которую надо возвести число `a`, чтобы получить число `x`

log_2 64 = 6 так как 2^6=64

Довольно здорово. Подобные алгоритмы уменьшают вдвое объем данных на каждой итерации. 
Если у вас 100 элементов, то чтобы найти ответ, потребуется около 7 шагов. 
При наличии 1000 элементов требуется 10 шагов. 
А для 1 000 000 элементов требуется всего 20 шагов. 
Это очень быстро даже для больших объемов данных. 
Пример: бинарный поиск.
*/
fn algo_2(mut decimal:u8) -> Option<String>{
    if decimal == 0 {return None;}
    let mut binary = String::from(""); 
    while decimal > 0 {
        binary = format!("{}{}",decimal%2,binary);
        decimal = decimal.div_floor(2);
    }
    Some(binary)
}

// или
let j = 1
while j < n {
  // do constant time stuff
  j *= 2
}

```

```rust
/* 
Временная сложность алгоритма - корень из N (sqrt N)
условие выхода из цикла x^2 следовательно цикл прекратится когда x >= sqrt N

TODO: Корень `n-й` степени из числа `a` определяется как такое число `b`, что `b^n=a` 
Здесь `n` — натуральное число, называемое показателем корня (или степенью корня); 
как правило, оно больше или равно 2, потому что случай `n=1` не представляет интереса.

Пример: Корнями 2-й степени из числа 9 являются +/-3  т.е. `9 sqrt^2=3`  
так как 3^2=9,а `64 sqrt^3=4` так как 4^3=64
И график O(sqrt N) будет расти быстрее, следовательно медленнее работать чем график O(log N)
так как для N=64 => `log_2 64 = 2^6 = 64 => 6` это < `64 sqrt^2 = 8^2 = 64 => 8`
т.е. для log мы двойку возводим в нужную степень, 
а для sqrt нужно само число для возведения в квадрат 
*/
fn algo_sqrt(v:&Vec<i32>){
    let mut x = 0;
    while x*x < v.len() {
        x+=1;
    }
}
```

```rust
/* 
Временная сложность алгоритма - линейная O(N)
Время выполнения увеличивается пропорционально размеру задачи.

Хорошая производительность. Если у вас 100 элементов, это выполняет 100 единиц работы. 
При удвоении количества элементов алгоритм будет выполняться ровно в два раза дольше (200 ед. работы). 
Пример: последовательный поиск.
*/
fn algo_3(v:&Vec<i32>,n:i32) -> bool{
    for i in v{
        if i == &n{
            return true;
        }
    }
    false
}
```

```rust
// Временная сложность алгоритма - линейная O(N + M) 
fn algo_5(n:&Vec<i32>,m:&Vec<i32>,element:&i32) -> Option<i32>{
    let mut value:i32 = 0;
    for i_n in n{
       if i_n > element{
         value = *i_n;
       }
    }
    if value == 0{return None};
    for i_m in m {
        if &value == i_m{
            return Some(*i_m);
        }
    }
    None
}
```

```rust
/*
Временная сложность алгоритма - линеарифмическая O(N * M) 

Достойная производительность. Это немного хуже линейного, но не так уж плохо. 
Пример: самые быстрые алгоритмы сортировки общего назначения.
*/
fn algo_6(n:&Vec<i32>,m:&Vec<i32>) -> Option<i32>{
    for i_n in n{
        for i_m in m{
            if i_n == i_m{
                return Some(*i_m);
            }
        }
    }
    None
}
```

```rust
/* 
Временная сложность алгоритма - полиномиальная (квадратичная) сложность O(N^2)
 1 + 2 + 3 + 4 + ... + N => O(N^2)
 N-1 * N-1 = N^2
Часто встречается в алгоритмах с вложенными циклами. 

Как-то медленно. Если у вас 100 элементов, это 100^2 = 10 000 единиц работы. 
Удвоение количества предметов делает процесс медленнее в четыре раза 
(поскольку 2 в квадрате равно 4). 
Пример: алгоритмы, использующие вложенные циклы, такие как сортировка вставками.
*/
fn algo_6_bubble_sort(v:&mut Vec<i32>){
    for i in 0..v.len()-1 {
        for j in 0..v.len()-1 {
            if v[j] > v[j+1]{
                let swap = v[j];
                v[j]=v[j+1];
                v[j+1]=swap;
            }
        }
    }
}

// или
for i in 0..n {
  for j in 0..n {
    ...
  }
}
```

```rust
/* 
Временная сложность алгоритма - полиномиальная (кубическая) сложность O(N^3)

Шаг за пределы квадратичной сложности. 
По мере увеличения размера задачи время выполнения увеличивается еще быстрее. 
Это происходит в алгоритмах с тремя вложенными циклами, например в алгоритмах 
с трехмерными массивами.

Низкая производительность. Если у вас 100 элементов, это 100^3 = 1 000 000 единиц работы.
Удвоение входного размера делает его в восемь раз медленнее. Пример: умножение матрицы.
*/
for i in 0..n {
  for j in 0..n {
    for k in 0..n {
       ...
    }
  }
}

``` 


```rust
/* 
Временная сложность алгоритма - экспоненциальная O(2^N)
 2^0 + 2^1 + 2^3 + ... + 2^N => O(2^N)

Очень плохая производительность. 
Вы хотите избежать подобных алгоритмов, но иногда у вас нет выбора. 
Добавление всего лишь одного бита к входным данным удваивает время работы. 
Пример: задача коммивояжера.

 cargo +nightly run
*/
fn main() {
    aasert_eq!(Some("11111110"),algo_2(254));

    let mut v = vec![5,7,1,4];
    algo_7_bubble_sort(&mut v);
    assert_eq!(vec![1,4,5,7],v);
}
```

```rust
/* 
Временная сложность алгоритма - факториал O(n!)
Они в основном используются в перестановочных и комбинаторных задачах.

Невыносимо медленно. Буквально на то, чтобы что-то сделать, уходит миллион лет.
В задаче "Комиивояжора" - построение всех возможных маршрутов, 
используется в картах или просто рекурсия
*/
fn factorial(n: i32) {
    for i in 0..n {
       factorial(n - 1)
    }
}

```



### Links

[8 лучших алгоритмов, которые должен знать каждый программист](https://vc.ru/u/1389654-machine-learning/593476-8-luchshih-algoritmov-kotorye-dolzhen-znat-kazhdyy-programmist)

[AllAlgorithms](https://github.com/AllAlgorithms/rust) 

[geeksforgeeks.org](https://www.geeksforgeeks.org/fundamentals-of-algorithms/?ref=shm)

[Algorithmic complexity / Big-O / Asymptotic analysis](https://github.com/jwasham/coding-interview-university#algorithmic-complexity--big-o--asymptotic-analysis)

[Комбинаторика](https://brestprog.by/topics/combinatorics/)

[TheAlgorithms Rust](https://github.com/TheAlgorithms/Rust/tree/master)

[Visual algo](https://visualgo.net/en)

[Visual Algorithms](https://www.cs.usfca.edu/~galles/visualization/Algorithms.html)

[Algorithms e-maxx.ru](http://e-maxx.ru/algo/)

[Sorting-Algorithms on Rust](https://github.com/diptangsu/Sorting-Algorithms/tree/master/Rust)

[Sorting algorithm Wiki](https://en.wikipedia.org/wiki/Sorting_algorithm)

[Грокаем алгоритмы GitHub](https://github.com/egonSchiele/grokking_algorithms)

[Radix sort](https://www.youtube.com/watch?v=_KhZ7F-jOlI)

[swift-algorithm-club](https://github.com/kodecocodes/swift-algorithm-club/blob/master/Bubble%20Sort/README.markdown)

[Coursera Algorithms part 1](https://www.coursera.org/learn/algorithms-part1)

[Coursera Algorithms part 2](https://www.coursera.org/learn/algorithms-part2)

[neetcode roadmap](https://neetcode.io/roadmap)
