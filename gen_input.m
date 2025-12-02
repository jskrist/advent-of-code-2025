function gen_input(day)

arguments
    day = 0;
end

filename = uiputfile('*.txt', 'input file name', ['day_', num2str(day), '_test_1.txt']);

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
elseif(day == 2)
    num_entries = 25;
    starts = randi(100000, 1, num_entries);
    for idx = 1:num_entries
        start = starts(idx);
        stop = start + randi(ceil(start * 0.25), 1);
        fprintf(fid, '%d-%d', start, stop);
        if(idx < num_entries)
            fprintf(fid, ',');
        end
    end
end
end
