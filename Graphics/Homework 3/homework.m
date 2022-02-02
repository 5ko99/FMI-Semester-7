function homework
    clc;
    clear;
    xMax = 100;
    yMax = 100;
    M=zeros(xMax,yMax); 
    Ax=axes;
    image(M');
    set(Ax,'YDir','normal');
    color = 50;
    N = inputdlg("Number of circles=");
    N = round(str2double(cell2mat(N)));
    for i=1:N
        coord = ginput(1);
        xc = round(coord(:,1));
        yc = round(coord(:,2));
        R = inputdlg("Enter R");
        R = cell2mat(R);
        R = round(str2double(R));
        M = DrawBresCircle(xc,yc,R,M,color);
        image(M');
        set(Ax,'YDir','normal');
    end

    while true
        [x,y,button] = ginput(1);
        x = round(x);
        y = round(y);
        newColor = 	color+90;
        M = StackBoundaryFill1(x,y,newColor,color,M,Ax);
        image(M');
        set(Ax,'YDir','normal');
    end
end

function M = StackBoundaryFill1(x,y,fillColor,boundaryColor,M,Ax)
    if M(x,y) ~= boundaryColor && M(x,y) ~= fillColor
        M(x,y) = fillColor;
        image(M');
        set(Ax,'YDir','normal');
        pause(0.002);
        M = StackBoundaryFill1(x+1,y,fillColor,boundaryColor,M,Ax);
        M = StackBoundaryFill1(x,y+1,fillColor,boundaryColor,M,Ax);
        M = StackBoundaryFill1(x-1,y,fillColor,boundaryColor,M,Ax);
        M = StackBoundaryFill1(x,y-1,fillColor,boundaryColor,M,Ax);
    end 
end

function M = DrawBresCircle(xc,yc,R,M,color)
    x = 0;
    y = R;
    d = 3-2*R;
    M = EightSymetric(xc,yc,x,y,color,M);
    while(y>=x)
        x = x+1;
        if(d>0)
            y = y - 1;
            d = d + 4*(x-y)+10;
        else
            d = d + 4 * x + 6;
        end
        M=EightSymetric(xc,yc,x,y,color,M);
    end
end


function M = EightSymetric(xc,yc,x,y,color,M)
    M = FourSymetric(xc,yc,x,y,color,M);
    M = FourSymetric(xc,yc,y,x,color,M);
end

function M = FourSymetric(xc,yc,x,y,color,M)
    M(xc+x,yc+y) = color;
    M(xc-x,yc-y)= color;
    M(xc-x,yc+y) = color;
    M(xc+x,yc-y) = color;
end