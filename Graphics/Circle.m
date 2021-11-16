function DrawCircle
    cla reset;
    clc
    grid on;
    xMax = 20;
    yMax = 20;
    axis([0 xMax 0 yMax]);
    coord = ginput(1);
    xc = round(coord(:,1));
    yc = round(coord(:,2));
    R = inputdlg("Enter R");
    R = cell2mat(R);
    R = round(str2double(R));
    DrawBresCircle(xc,yc,R,'r.')

function DrawBresCircle(xc,yc,R,val)
    x = 0;
    y = R;
    d = 3-2*R;
    EightSymetric(xc,yc,x,y,val)
    while(y>=x)
        x = x+1;
        if(d>0)
            y = y - 1;
            d = d + 4*(x-y)+10;
        else
            d = d + 4 * x + 6;
        end
        EightSymetric(xc,yc,x,y,val)
    end


function EightSymetric(xc,yc,x,y,val)
    FourSymetric(xc,yc,x,y,val)
    hold on;
    FourSymetric(xc,yc,y,x,val)
    hold on;

function FourSymetric(xc,yc,x,y,val)
    plot(xc+x,yc+y,val)
    hold on;
    plot(xc-x,yc-y,val)
    hold on;
    plot(xc-x,yc+y,val)
    hold on;
    plot(xc+x,yc-y,val)
    hold on;
    axis([0 20 0 20]);
    grid on;
    hold on;
