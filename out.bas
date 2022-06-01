10 DIM A0(7)
20 LET A1 = 0
30 
40 
50 goto 120
60 
70 return
80 gosub 80
90 gosub 60
100 A1 = 7
110 return
120 gosub 80
130 A0(7) = A1
140 END
