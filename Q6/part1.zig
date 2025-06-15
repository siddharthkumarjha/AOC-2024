const std = @import("std");
const Part1_Impl = @import("./part1_impl.zig");
const string = Part1_Impl.string;
var allocator = std.heap.page_allocator;
const part1 = Part1_Impl.part1;

// override log level
pub const std_options = std.Options{
    .log_level = .debug,
};

pub fn main() !void {
    var buffer: string = string.init(allocator);
    defer buffer.deinit();

    const distinct_pos_visited = try part1(allocator, &buffer, null);

    std.log.debug("\n{s}\n", .{buffer.items});
    std.log.debug("======================================================", .{});
    std.log.debug("no of distinct positions visited: {}", .{distinct_pos_visited});
}
