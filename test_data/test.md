---
id: eigenschaften_von_teilmengen__subsets_
aliases:
  - Eigenschaften von Teilmengen (Subsets)
  - Compact Set
  - Kompaktes Set
  - Bounded Set
  - Closed Set
  - Open Set
tags:
  - Analysis
---

# Eigenschaften von Teilmengen (Subsets)

## Definition: Bounded #card

A subset $X\subseteq \mathbb{R}^n$ is _bounded_ if the set of $||x||$ for $x\in X$ is bounded in $\mathbb{R}$

Also wenn die Länge der Vektoren im Set $X$ beschränkt ist.

[source](assets/pdfs/analysis_2/script.pdf?section="Definition%203.2.11")

## Definition: Closed #card

A subset $X \subseteq \mathbb{R}^n$ is _closed_ if for every sequence $(x_k)_{k\geq 1}$ in $X$ that converges in $\mathbb{R}^n$ to some vector $y\in \mathbb{R}^n$, we have $y\in X$

[source](assets/pdfs/analysis_2/script.pdf?section="Definition%203.2.11")

### Beispiele:

$$\{(x,y) \in \mathbb{R}^2 : x^2 + y^2 \leq 2\}$$

Die Menge ist Abgeschlossen, weil der Rand dazugehört. Dadurch sind alle Folgen drinnen enthalten.

## Definition: Open #card

Ein Subset ist **open** genau dann, wenn es immer einen Radius $\sigma$ um jeden Punkt aus dem Subset gibt, so dass alle Punkte in dem Radius auch in dem Subset beinhaltet sind. (Siehe [Abgeschlossene Intervalle](Abgeschlossene%20Intervalle.md))

> Formal:
>
> A subset $x \ subseteq \mathbb{R}^n$ is **open** if, for any $x = (x_1, \dots, x_n) \in X$ there exists $\sigma > 0$ such that the set
> $$\{y = (y_1, \dots, y_n) \in \mathbb{R}^n : | x_i - y_i | < \sigma \text{ for all } i \}$$
> is contained in X.

In other words: Any point in $\mathbb{R}^n$ obtained by changing any coordinate of $x$ by at most $\sigma$ is still in $X$.

### Lemma: Complement is closed #card

A set $X \subseteq \mathbb{R}$ is open **if and only if** the complement
$$Y = \{x \in \mathbb{R}^n : x \notin X\}$$
is closed.

### Lemma: Urbild is closed #card

If $f: \mathbb{R}^n \to \mathbb{R}^n$ is [continuous](stetigkeit_im_mehrdimensionalen.md) and $Y \subseteq \mathbb{R}^m$ is open, then $f^{-1}(Y)$ is open in $\mathbb{R}^n$

### Special Open Sets

The sets

- $\mathbb{R}^n$
- $\varnothing$
  are the only sets that are **open and closed**

### Lemma: Intervals and Open Sets #card

Let $I_1, \dots, I_n$ be open intervals $\mathbb{R}$. Then $I_1, \dots, I_n$ is open in $\mathbb{R}^n$

### Beispiele:

$$\{(x,y) \in \mathbb{R}^2 : x^2 + y^2 < 2\}$$

Die Menge ist Offen, weil der nicht dazugehört. Dadurch können wir eine Folge finden, welche den Rand als Grenzwert hat.

---

Die Menge $\mathbb{Q}$ ist nicht closed und nicht open, da man eine Folge finden kann die zu $\sqrt{2}$ konvergiert finden kann und man kann keinen Radius drum herum legen, weil es drum herum immer Zahlen in $\mathbb{R}$ gibt, welche man nicht hineinpacken kann.

## Definition: Compact #card

A subset $X \subseteq \mathbb{R}^n$ is _compact_ if it is bounded and closed

[source](assets/pdfs/analysis_2/script.pdf?section="Definition%203.2.11")

### Lemma: Closed Set on a continuous map #card

Wenn $f:\mathbb{R}^n \to \mathbb{R}^m$ eine [continous](stetigkeit_im_mehrdimensionalen.md) [map](linear_map.md) ist und $Y \subseteq \mathbb{R}^m$ ein _closed_ set ist, dann gilt

$$f^{-1}(Y)=\{x\in\mathbb{R}^n \quad|\quad f(x)\in Y\}\subseteq\mathbb{R}^n$$

### Lemma: Bounded Functions and maximum und minimum #card

Wenn wir ein kompaktes Set auf eine [stetige Funktion](stetigkeit_im_mehrdimensionalen.md) anwenden, dann ist das **Bild** der Funktion auch bounded und die Funktion hat ein Maximum und ein Minimum.

$$f(x_+)=\sup_{x\in X}f(x) \quad f(x_-)=\inf_{x\in X}f(x)$$
