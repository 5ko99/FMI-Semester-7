function Cirlce
    clc;
    clear;
    xMax = 50;
    yMax = 50;
    M=zeros(xMax,yMax); 
    Ax=axes;
    image(M');
    set(Ax,'YDir','normal');
    color = 250;
    set(Ax,'YDir','normal');
    coord = ginput(1);
    xc = round(coord(:,1));
    yc = round(coord(:,2));
    R = inputdlg("Enter R");
    R = cell2mat(R);
    R = round(str2double(R));
    M = FillCircle(xc,yc,R,color,M);
    %pause(2);
    %M = DrawBresCircle(xc,yc,R,M,color*50);
end

function M = DrawBresCircle(xc,yc,R,M,color)
    x = 0;
    y = R;
    d = 3-2*R;
    M = EightSymetric(xc,yc,x,y,color,M);
    image(M');
    pause(0.7);
    while(y>=x)
        x = x+1;
        if(d>0)
            y = y - 1;
            d = d + 4*(x-y)+10;
        else
            d = d + 4 * x + 6;
        end
        M=EightSymetric(xc,yc,x,y,color,M);
        image(M');
        pause(0.7);
    end
    return;
end


function M = FillCircle(xc,yc,R,color,M)
    x = 0;
    y = R;
    d = 3-2*R;
    M = EightSymetricLine(xc,yc,x,y,color*13,M);
    image(M');
    pause(0.7);
    while(y>x)
        x = x+1;
        if(d>0)
            y = y - 1;
            d = d + 4*(x-y)+10;
        else
            d = d + 4 * x + 6;
        end
        M = EightSymetricLine(xc,yc,x,y,color+8*x+8*y,M);
        image(M');
        pause(0.7);
    end
    return;
end


function M = EightSymetric(xc,yc,x,y,color,M)
    M = FourSymetric(xc,yc,x,y,color,M);
    %hold on;
    M = FourSymetric(xc,yc,y,x,color,M);
   % hold on;
    return;
end

function M = FourSymetric(xc,yc,x,y,color,M)
    %plot(xc+x,yc+y,val)
    M(xc+x,yc+y) = color;
    %hold on;
    %plot(xc-x,yc-y,val)
    M(xc-x,yc-y)= color;
    %hold on;
    %plot(xc-x,yc+y,val)
    M(xc-x,yc+y) = color;
    %hold on;
   % plot(xc+x,yc-y,val)
    M(xc+x,yc-y) = color;
    %hold on;
    %axis([0 20 0 20]);
    %grid on;
   % hold on;
   return;
end

function M = EightSymetricLine(xc,yc,x,y,color,M)
    M = FourSymetricLine(xc,yc,x,y,color,M);
    M = FourSymetricLine(xc,yc,y,x,color,M);
    return;
end

function M = FourSymetricLine(xc,yc,x,y,color,M)
    M = Line(xc-x,yc+y,xc+x,yc+y,color,M);
    M = Line(xc-x,yc-y,xc+x,yc-y,color,M);
    return;
end

function M = Line(x0,y0,x1,y1,color,M)
incX = 1;
incY = 1;
reverse = false;

H = abs(x1-x0);
V = abs(y1-y0);
if x1 < x0
    incX = -1;
end
if y1 < y0
    incY = -1;
end

if H < V
    reverse = true;
    i = H;
    H = V;
    V = i;
end

incUp = 2 * V - 2 * H;
incDn = 2 * V;
Est = 2 * V - H;

X = x0;
Y = y0;
for i=0:H
    M(X,Y) = color;
    if Est >= 0
        Est = Est + incUp;
        X = X + incX;
        Y = Y + incY;
    else
        Est = Est + incDn;
        if reverse == true
            Y = Y + incY;
        else
            X = X + incX;
        end
    end
end
end