# Tests for a(n)b(n)c(n), i.e. abc, aabbcc, aaabbbccc but not ba, acab, abbbcc, cba, etc.

# Alphabet
[a, b, c]

q0!
{ 	a > q1 x
,	y > q5 _
}

q1
{	a > _
,	b > q2 y
,	y > _
}

q2
{	b > _
,	c > q3 z
,	z > q2 _
} 

q3
{	c > _
,	< q4 _
}

q4
{	c < _
,	b < _
,	a < _
,	z < _
,	y < _
,	x > q0 _
}

q5
{	c < q6 _
,   b < q6 _
,   y > _
,   z > _
, 	< q7 _
}

q6

q7!
