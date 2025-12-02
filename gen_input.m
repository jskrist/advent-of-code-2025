function gen_input(day)

arguments
    day = 0;
end

filename = uiputfile('*.txt', 'input file name', ['input_day_', num2str(day), '.txt']);

if(isnumeric(filename))
    return
end

fid = fopen(filename, 'w');
clean.file = onCleanup(@()fclose(fid));

if(day <= 1)
    num_entries = 6;
    distances = strsplit(num2str(randi(80, 1, num_entries)));
    direction_options = 'RL';
    directions = cellstr(direction_options(randi(2, num_entries, 1))')';
    
    data = strcat(directions, distances);
    fprintf(fid, '%s\n', data{:});
end

end