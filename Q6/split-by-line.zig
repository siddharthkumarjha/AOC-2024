const std = @import("std");
pub const string = std.ArrayList(u8);
pub const lines = std.ArrayList([]u8);
pub const Allocator = std.mem.Allocator;

pub fn SplitIterator(comptime T: type) type {
    return struct {
        buffer: []T,
        index: ?usize,
        delimiter: T,

        const Self = @This();

        /// Returns a slice of the next field, or null if splitting is complete.
        pub fn next(self: *Self) ?[]T {
            const start = self.index orelse return null;
            const end = if (std.mem.indexOfScalarPos(T, self.buffer, start, self.delimiter)) |delim_start| blk: {
                self.index = delim_start + 1;
                break :blk delim_start;
            } else blk: {
                self.index = null;
                break :blk self.buffer.len;
            };
            return self.buffer[start..end];
        }
    };
}

pub fn splitByLine(comptime T: type, buffer: []T) SplitIterator(T) {
    return .{
        .index = 0,
        .buffer = buffer,
        .delimiter = '\n',
    };
}

pub fn convertToSliceOfLines(allocator: Allocator, buffer: *const string) ![][]u8 {
    var array_of_lines: lines = lines.init(allocator);
    defer array_of_lines.deinit();

    var itr_lines = splitByLine(u8, buffer.items);
    while (itr_lines.next()) |line| {
        if (line.len > 0) {
            try array_of_lines.append(line);
        }
    }

    return try array_of_lines.toOwnedSlice();
}
