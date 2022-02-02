%ChangeNode
drMax = 1000;
selectedNode = 0;
drOld = 1e6;
[X,Y]=ginput(1);
for i=1:n
    delX=XBin(i)-round(X);
    delY=YBin(i)-round(Y);
    drNew=delX^2+delY^2;
    if drNew < drMax && drNew < drOld
        selectedNode=i;
    end
    drOld=drNew;
end
if selectedNode
    rectangle('Curvature',[1 1],'Position',...
        [XBin(selectedNode)-nodeSize,YBin(selectedNode)-nodeSize,...
        2*nodeSize,2*nodeSize],'FaceColor',[0,1,0]);
    [X,Y]=ginput(1);
    clf;
    Ax=axes('Position',[0.1,0.1,0.8,0.8],'XLim',[0 MaxX],'YLim',[0 MaxY]);
    XBin(selectedNode)=round(X);
    YBin(selectedNode)=round(Y);
    for i=1:n
        rectangle('Curvature',[1 1],'Position',[XBin(i)-nodeSize,...
            YBin(i)-nodeSize,2*nodeSize,2*nodeSize],'FaceColor',[0,0,1]);
    end
    interpolationOverhauser;
end