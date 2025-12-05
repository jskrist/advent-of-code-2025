function gen_input(day)

arguments
    day = 0;
end

[filename, filepath] = uiputfile('*.txt', 'input file name', ['day_', num2str(day), '_test_1.txt']);

if(isnumeric(filename))
    return
end

fid = fopen(fullfile(filepath, filename), 'w');
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
elseif(day == 3)
    num_digits = 100;
    num_rows = 200;
    data = randi(10, num_rows, num_digits) - 1;
    zero_idx = data(:, 1) == 0;
    while(any(zero_idx))
        data(zero_idx, 1) = randi(10, sum(zero_idx), 1) - 1;
        zero_idx = data(:, 1) == 0;
    end
    for idx = 1:num_rows
        fprintf(fid, '%d', data(idx, :));
        fprintf(fid, '\n');
    end
elseif(day == 4)
    num_rows = 135;
    num_cols = 135;
    data = randi(2, num_rows, num_cols) - 1;
    output = char(data);
    output(data == 0) = '.';
    output(data == 1) = '@';
    for idx = 1:num_rows
        fprintf(fid, '%s', output(idx, :));
        fprintf(fid, '\n');
    end
elseif(day == 5)
    num_ranges = 179;
    num_ids = 1000;
    max_id_start = 6E11;
    max_range_range = ceil(max_id_start*0.50);
    start_val = sort(randi(max_id_start, 1, num_ranges));
    stop_val = start_val + randi(max_range_range, 1, num_ranges);
    ids = [];
    while(numel(ids) < num_ids)
        ids = [ids, unique(randi(max_id_start + max_range_range, 1, num_ids - numel(ids)))]; %#ok<AGROW>
    end
    
    for idx = 1:num_ranges
        fprintf(fid, '%d-%d\n', start_val(idx), stop_val(idx));
    end
    fprintf(fid, '\n');
    fprintf(fid, '%d\n', ids);
end
end
