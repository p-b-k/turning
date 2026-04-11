# The alphabet
[0, 1]

# The Initial State
q0!
{	1 > q1 a
}

q1
{	1 > q2 a
,	0 < q9 0
}

q2
{	1 > _
,	0 > q3 _
}

q3
{	1 > _
,	< q4
,	b > _
}

q4
{	1 > q5 b
,	b < _
,   c < _
, 	0 > q6 0
}

q5
{	< q4 c
, 	b > _
, 	c > _
,  	1 > _
}

q6
{	b > _
, 	c > q6 1
, 	< q7
}

q7
{	1 < _
,	b < q7 1
, 	0 < q8 _
}

q8
{	1 < _
,	a > q1 _
}

q9
{	a < _
,	> q10
}

q10
{	a > q10
,	b > q11 1
,	1 > q11 1
,	0 > q11
}

q11!

