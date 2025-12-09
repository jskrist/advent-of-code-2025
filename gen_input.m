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
elseif(day == 6)

    num_problems = 4;
    num_nums = 3;
    num_digits = randi([2, 4], 1, num_problems);
    max_vals = 10 .^num_digits;
    num_format_str = sprintf('%%-%dd ', num_digits);
    num_format_str = [num_format_str(1:end-1) , '\n'];
    minus_ind = strfind(num_format_str, '-');
    minus_ind(randperm(num_problems, floor(num_problems/2))) = [];
    num_format_str(minus_ind) = [];

    ops = {'+', '+', '+', '+', '*'}; % bias towards addition


    distrib_stepsize = [0.2000, 0.0476, 0.0270, 0.0192, 0.0152, 0.0128, 0.0114, 0.0105, 0.0101, 0.0100, 0.0100, 0.0270, 0.0714, ...
                        0.2000, 0.5000, ones(1, 80)] * 0.0001;
    distribution = cell(1, numel(distrib_stepsize));
    start = 0;
    dist = 1 / numel(distrib_stepsize);
    for idx = 1:numel(distrib_stepsize)
        stop = start + dist;
        distribution{idx} = start:distrib_stepsize(idx):stop;
        start = stop;
    end
    distribution = [distribution{:}];

    data = NaN(num_nums, num_problems);
    for idx = 1:num_problems
        this_distrib = distribution * (max_vals(idx)-1);
        data(:, idx) = max(floor(this_distrib(randperm(numel(this_distrib), num_nums))), 1);
    end
    % make sure there is at least one value that spans the col width
    low_max_ind = find(max(data) < max_vals/10);
    for idx = low_max_ind
        [~, max_ind] = max(data(:, idx));
        data(max_ind, idx) = max_vals(idx)/2;
    end
    % sort to avoid gaps for part 2
    data = sort(data);

    fprintf(fid, num_format_str, data');

    op_format_str = sprintf('%%-%dc ', num_digits);
    op_format_str = [op_format_str(1:end-1) , '\n'];
    all_ops = ops(randi(numel(ops), 1, num_problems));
    fprintf(fid, op_format_str, all_ops{:});
elseif(day == 7)
    num_rows = 142;
    num_cols = num_rows - 1;
    mid_point = ceil(num_cols/2);

    all_data = repmat('.', num_rows, num_cols);
    all_data(1, mid_point) = 'S';
    for level = 1:(floor(num_rows/2)- 1)
        num_splits = level;
        
        split_ind = (mid_point - (num_splits-1)):2:(mid_point + (num_splits - 1)) ;
        all_data((level*2 + 1), split_ind) = '^';
    end
    shift_amt = randi(3, 1, floor(num_cols/2)-1) - 2;
    split_levels = 3:2:num_rows;
    for idx = 1:numel(shift_amt)
        all_data(split_levels(idx), :) = circshift(all_data(split_levels(idx), :), shift_amt(idx));
    end

    % print all data
    fprintf(fid, '%s', strjoin(cellstr(all_data), newline));
elseif(day == 8)
    num_coords = 1000;
    max_coord = 100000;
    coords = randi(max_coord, num_coords, 3);
    for idx = 1:num_coords
        fprintf(fid, '%d,%d,%d', coords(idx, :));
        if(idx < num_coords)
            fprintf(fid, '\n');
        end
    end
end
end
