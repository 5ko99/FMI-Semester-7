function Main
    clc;
    clear;
    xMax = 400;
    yMax = 400;
    M=zeros(xMax,yMax); 
    Ax=axes;
    image(M');
    set(Ax,'YDir','normal');
    color = 50;
    set(Ax,'YDir','normal');
    coord = ginput(2);
    x = round(coord(:,1));
    y = round(coord(:,2));
    x1 = x(1,:);
    x2 = x(2,:);
    y1 = y(1,:);
    y2 = y(2,:);
    
    if x1 > x2
        xMax = x1;
        xMin = x2;
    else
        xMax = x2;
        xMin = x1;
    end

    if y1>y2
        yMax = y1;
        yMin = y2;
    else
        yMax = y2;
        yMin = y1;
    end

    M = Line(x1,y1,x2,y1,color,M);
    M = Line(x2,y1,x2,y2,color,M);
    M = Line(x1,y2,x2,y2,color,M);
    M = Line(x1,y2,x1,y1,color,M);


    coord = ginput(2);
    x = round(coord(:,1));
    y = round(coord(:,2));
    x1 = x(1,:);
    x2 = x(2,:);
    y1 = y(1,:);
    y2 = y(2,:);
    color = color + 70;
    M = Line(x1,y1,x2,y2,color,M);
    
    
    color = color +125;
    M1 = CohenShuttherLand2DClip(x1,y1,x2,y2,xMax,yMax,xMin,yMin);
    x1 = round(M1(1));
    y1 = round(M1(2));
    x2 = round(M1(3));
    y2 = round(M1(4));

    xMin
    yMin
    xMax
    yMax
    
    x1
    y1
    x2
    y2
    
    if x1 ~= 0 && y1 ~=0 && x2~=0 && y2~=0
        pause(1);
        M = Line(x1,y1,x2,y2,color,M);
    end

end

function code = generateCode(x,y,xMax,yMax,xMin,yMin)
    LEFT   = 1;
    RIGHT  = 2;
    INSIDE = 0;
    BOTTOM = 4;
    TOP    = 8;

    code = INSIDE;
    
    if x < xMin
        code = bitor(code,LEFT);
    elseif x > xMax
        code = bitor(code,RIGHT);
    end

    if y<yMin
        code = bitor(code,BOTTOM);
    elseif y>yMax
        code = bitor(code,TOP);
    end
end

function M = CohenShuttherLand2DClip(x1,y1,x2,y2,xMax,yMax,xMin,yMin)
    LEFT   = 1;
    RIGHT  = 2;
    INSIDE = 0;
    BOTTOM = 4;
    TOP    = 8;

    code1 = generateCode(x1,y1,xMax,yMax,xMin,yMin);
    code2 = generateCode(x2,y2,xMax,yMax,xMin,yMin);
    accept = false;

    while true
        if not(bitor(code1,code2))
            %both endpoints are inside.
            accept=true;
            break;
        elseif bitand(code1,code2)
            %both endpoints are outside.
            accept = false;
            break;
        else
            if code2 > code1
                code_out = code2;
            else
                code_out = code1;
            end
            
            if bitand(code_out,TOP)
                x = x1 + (x2 - x1) * (yMax - y1) / (y2 - y1);
                y = yMax;
            elseif bitand(code_out,BOTTOM)
                x = x1 + (x2 - x1) * (yMin - y1) / (y2 - y1);
                y = yMin;
            elseif bitand(code_out,RIGHT)
                y = y1 + (y2 - y1) * (xMax - x1) / (x2 - x1);
                x = xMax;
            elseif bitand(code_out,LEFT)
                y = y1 + (y2 - y1) * (xMin - x1) / (x2 - x1);
                x = xMin;
            end

            if code_out == code1
                x1 = x;
                y1 = y;
                code1 = generateCode(x1,y1,xMax,yMax,xMin,yMin);
            else
                x2 = x;
                y2 = y;
                code2 = generateCode(x2,y2,xMax,yMax,xMin,yMin);
            end
        end
    end
    if accept
        M = [x1,y1,x2,y2];
    else
        M = [0,0,0,0];
    end
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