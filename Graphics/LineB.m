function LineB()
fig = uifigure;
clearButton = uibutton(fig,'push', 'Text','Clear','Position',[250,210,135,25], 'ButtonPushedFcn', @(btn,event) clearPlot());
butPlotSimple = uibutton(fig,'push', 'Text','Plot Simple','Position',[250,240,135,25], 'ButtonPushedFcn', @(btn,event) LineBHelp([]));
butPlotRound = uibutton(fig,'push', 'Text','Plot Round method','Position',[250,270,135,25], 'ButtonPushedFcn', @(btn,event) LineBRound([]));
butPlotTwo = uibutton(fig,'push', 'Text','Plot Two methods','Position',[250,300,135,25], 'ButtonPushedFcn', @(btn,event) RunBoth());
exitButton = uibutton(fig,'push', 'Text','Exit','Position',[0,0,135,25], 'ButtonPushedFcn', @(btn,event) quit);

function clearPlot()
    cla reset;
    clc;
    grid on;
    xMax = 80;
    yMax = 80;
    axis([0 xMax 0 yMax]);

function LineBHelp(coord)
clc
grid on;
xMax = 50;
yMax = 50;
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
hold on;
for i=0:H
    plot(curX,curY,'r.')
    if Est >= 0
        Est = Est + incUp;
        curX = curX + incX;
        curY = curY + incY;
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

function LineBRound(coord)
clc
grid on;
xMax = 50;
yMax = 50;
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

H = abs(x1-x0);
V = abs(y1-y0);

sx = x1 < x0;
sy = y1 < y0;
reverse = false;

if H < V
    reverse = true;
    x = H;
    H = V;
    V = x;
end

if H==0
    plot(x0,y0,'r*');
    return;
end

slope = V/H;

hold on;
for i=0:H
    x = i;
    y = round(slope * x+ 1/2);
    if reverse
        z = x;
        x = y;
        y = z;
    end
    if sx
        x = -x;
    end
    if sy
        y = - y;
    end
    plot(x+x0,y+y0,'r.');
end
axis([0 xMax 0 yMax]);
grid on;

function RunBoth()
clc
grid on;
xMax = 50;
yMax = 50;
axis([0 xMax 0 yMax]);
coord = ginput(2);
LineBHelp(coord)
LineBRound(coord)
