# Tests for a(n)b(n), i.e. ab, aabb, aaabbb but not ba, aab, abbb, abba, etc.

# Alphabet
[a, b]

# States and transitions
q0
{	a > q1 x
}

q1
{	a > _
,	y > _
,	b < q2 y
}

q2
{	y < _
,	a < _
,   x > q3 x
}

q3
{	a > q1 x
,	y > q4 _
}

q4
{	y > _
,	< q5 _
}

q5!


