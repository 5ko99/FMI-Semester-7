for ii=1:MaxX
    for jj=1:MaxY
        if M(ii,jj)==color
            rectangle('Curvature',[1 1],'Position',[ii-1,jj-1,1,1],'FaceColor',Col(color,:));
        end
    end
end
