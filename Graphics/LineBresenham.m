function Main
    clc;
    clear;
    xMax = 20;
    yMax = 20;
    M=zeros(xMax,yMax); 
    Ax=axes;
    image(M');
    set(Ax,'YDir','normal');
    color = 50;
    set(Ax,'YDir','normal');
    coord = ginput(2);
    x = round(coord(:,1));
    y = round(coord(:,2));
    x0 = x(1,:);
    x1 = x(2,:);
    y0 = y(1,:);
    y1 = y(2,:);
    M = Line(x0,y0,x1,y1,color,M);
    pause(1.5);
    color = 250;
    %coord = ginput(2);
   % x = round(coord(:,1));
   % y = round(coord(:,2));
   % x0 = x(1,:);
   % x1 = x(2,:);
   % y0 = y(1,:);
   % y1 = y(2,:);
    M = TwoEndsToCenter(x0,y0,x1,y1,color,M);
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
hold on;
for i=0:H
    M(X,Y) = color;
    image(M');
    pause(0.7);
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

function M = TwoEndsToCenter(x0,y0,x1,y1,color,M)
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

curX = x0;
curY = y0;
curZ = x1;
curT = y1;

for i=0:H/2
    M(curX,curY) = color;
    M(curZ,curT) = color;
    image(M');
    pause(0.7)
    if Est >= 0
        Est = Est + incUp;
        curX = curX + incX;
        curY = curY + incY;
        curZ = curZ - incX;
        curT = curT - incY;
    else
        Est = Est + incDn;
        if reverse == true
            curY = curY + incY;
            curT = curT - incY;
        else
            curX = curX + incX;
            curT = curT - incX;
        end
    end
end
end