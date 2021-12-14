%Растеризиране на отсечка по метода със закръгляне(tema1_round)
%int X1,Y1,X2,Y2;parametri
%int d,dx,dy,x,inty
%int incX,n,reverse; float y, incY;
dx=abs(X2-X1);
dy=abs(Y2-Y1);
reverse=(dx<dy);
if reverse
    d=X1;
    X1=Y1;
    Y1=d;
    d=X2;
    X2=Y2;
    Y2=d;
    d=dx;
    dx=dy;
    dy=d;
end
if (X1<=X2)
    incX=1;
else
    incX=-1;
end
incY=(Y2-Y1)/dx;
x=X1;
y=Y1;
n=dx+1;
while n
    inty=round(y);
    if (reverse)
        M(inty,x)=color;
    else
        M(x,inty)=color;
    end
    x=x+incX;
    y=y+incY;
    n=n-1;
end