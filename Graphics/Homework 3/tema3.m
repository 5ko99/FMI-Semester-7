clear
MaxX=100;MaxY=70;
M=zeros(MaxX,MaxY);
Col=[1,1,1;0,0,0];
color=2;
fprintf('Vavedete varhovete na oblastta: \n');
fprintf('(Kraj-natiskane na desen buton na mishkata) \n');
pause(2);
Ax=axes('Position',[0.1,0.1,0.8,0.8],'XLim',[0 100],'YLim',[0 70]);
i=1;
[u(1),v(1),B]=ginput(1);
u(1)=round(u(1));
v(1)=round(v(1));
while (B~=3)
    i=i+1;
    [u(i),v(i),B]=ginput(1);  
    u(i)=round(u(i));
    v(i)=round(v(i));
    X1=u(i-1); Y1=v(i-1);
    X2=u(i); Y2=v(i);
    tema1_round;
    tema3_draw;
end
    X1=u(1); Y1=v(1);
    X2=u(i); Y2=v(i);
    tema1_round;
    tema3_draw;