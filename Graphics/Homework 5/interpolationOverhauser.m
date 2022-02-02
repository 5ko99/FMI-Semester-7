%interpolationOverhauser
for i = 1:(n-3)
    KXY = [XBin(i),YBin(i)];
    LXY = [XBin(i+1),YBin(i+1)];
    MXY = [XBin(i+2),YBin(i+2)];
    NXY = [XBin(i+3),YBin(i+3)];
    a11 = KXY(1);
    a12 = KXY(2);
    b11 = 4.0*(LXY(1)-a11)-(MXY(1)-a11);
    b12 = 4.0*(LXY(2)-a12)-(MXY(2)-a12);
    c11 = 4.0*(a11-LXY(1))+2.0*(MXY(1)-a11);
    c12 = 4.0*(a12-LXY(2))+2.0*(MXY(2)-a12);
    if i==1
        oldX = a11;
        oldY = a12;
        for j = 1:tResolution
            t = 0.5*j/tResolution;
            newX = round(a11+b11*t+c11*t^2);
            newY = round(a12+b12*t+c12*t^2);
            if abs(newX-oldX) || abs(newY-oldY)
                rectangle('Curvature',[0 0],'Position',[newX-1,newY-1,1,1],...
                    'FaceColor',color);
            end
            oldX = newX;
            oldY = newY;
        end
    end
    a21 = LXY(1);
    a22 = LXY(2);
    b21 = 4.0*(MXY(1)-a21)-(NXY(1)-a21);
    b22 = 4.0*(MXY(2)-a22)-(NXY(2)-a22);
    c21 = 4.0*(a21-MXY(1))+2.0*(NXY(1)-a21);
    c22 = 4.0*(a22-MXY(2))+2.0*(NXY(2)-a22);
    if i==n-3
        oldX = a21;
        oldY = a22;
        for j = 1:tResolution
            t = 0.5+0.5*j/tResolution;
            newX = round(a21+b21*t+c21*t^2);
            newY = round(a22+b22*t+c22*t^2);
            if abs(newX-oldX) || abs(newY-oldY)
                rectangle('Curvature',[0 0],'Position',[newX-1,newY-1,1,1],...
                    'FaceColor',color);
            end
            oldX = newX;
            oldY = newY;
        end
    end
    t = 0;
    oldX = a21;
    oldY = a22;
    for j = 1:tResolution
        t = 0.5*j/tResolution;
        s = t + 0.5;
        alpha = 1.0-2.0*t;
        beta = 2.0*t;
        newX = round(alpha*(a11+b11*s+c11*s^2)+beta*(a21+b21*t+c21*t^2));
        newY = round(alpha*(a12+b12*s+c12*s^2)+beta*(a22+b22*t+c22*t^2));
        if abs(newX-oldX) || abs(newY-oldY)
            rectangle('Curvature',[0 0],'Position',[newX-1,newY-1,1,1],...
                'FaceColor',color);
        end
        oldX = newX;
        oldY = newY;
    end
end