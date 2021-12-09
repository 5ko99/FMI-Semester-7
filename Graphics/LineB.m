function LineB()
fig = uifigure;
clearButton = uibutton(fig,'push', 'Text','Clear','Position',[200,210,165,25], 'ButtonPushedFcn', @(btn,event) clearPlot());
butPlotSimple = uibutton(fig,'push', 'Text','Plot Bresenham','Position',[200,240,165,25], 'ButtonPushedFcn', @(btn,event) Bresenham([]));
butPlotRound = uibutton(fig,'push', 'Text','Plot from two ends to center','Position',[200,270,165,25], 'ButtonPushedFcn', @(btn,event) TwoEndsToCenter([]));
butPlotTwo = uibutton(fig,'push', 'Text','Plot Two methods','Position',[200,300,165,25], 'ButtonPushedFcn', @(btn,event) RunBoth());
exitButton = uibutton(fig,'push', 'Text','Exit','Position',[5,5,165,25], 'ButtonPushedFcn', @(btn,event) quit);

function clearPlot()
    cla reset;
    clc;
    grid on;
    xMax = 12;
    yMax = 12;
    axis([0 xMax 0 yMax]);

function TwoEndsToCenter(coord)
clc
grid on;
xMax = 12;
yMax = 12;
axis([0 xMax 0 yMax]);

if isempty(coord)
    coord = ginput(2);
end

x = round(coord(:,1));
y = round(coord(:,2));
x0 = x(1,:);
x1 = x(2,:);
y0 = y(1,:);
y1 = y(2,:);
incX = 1;
incY = 1;
reverse = false;
H = abs(x1-x0);
V = abs(y1-y0);
i= 0;
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
hold on;
for i=0:H/2
    plot(curX,curY,'b*')
    plot(curZ,curT,'b*')
    pause(1)
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
        else
            curX = curX + incX;
        end
    end
end

axis([0 xMax 0 yMax]);
grid on;

function Bresenham(coord)
clc
grid on;
xMax = 12;
yMax = 12;
axis([0 xMax 0 yMax]);

if isempty(coord)
    coord = ginput(2);
end

x = round(coord(:,1));
y = round(coord(:,2));
x0 = x(1,:);
x1 = x(2,:);
y0 = y(1,:);
y1 = y(2,:);

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
    plot(X,Y,'r*')
    pause(1);
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
axis([0 xMax 0 yMax]);
grid on;


function RunBoth()
clc
grid on;
xMax = 12;
yMax = 12;
axis([0 xMax 0 yMax]);
coord = ginput(2);
TwoEndsToCenter(coord)
Bresenham(coord)
