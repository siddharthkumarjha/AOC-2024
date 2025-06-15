const std = @import("std");
const splitByLine = @import("./split-by-line.zig");

pub const string = splitByLine.string;
pub const lines = splitByLine.lines;
pub const Allocator = std.mem.Allocator;

const Direction = enum { Up, Down, Left, Right };
pub const GuardPos = struct {
    line: usize,
    col: usize,
};
const MoveResult = enum { OutOfBounds, ObstacleCollision };
const MoveConst = struct {
    const Visited = 'X';
    const Obstacle = '#';
    const Guard = '^';
};

const allocator = std.heap.page_allocator;
const cwd = std.fs.cwd();

fn findPosOfGuard(array_of_lines: *lines) ?GuardPos {
    for (array_of_lines.items, 0..) |line, line_idx| {
        if (std.mem.indexOfScalar(u8, line, MoveConst.Guard)) |index_of_guard| {
            return GuardPos{
                .line = line_idx,
                .col = index_of_guard,
            };
        }
    }
    return null;
}

fn MoveRight(array_of_lines: *lines, pos: *GuardPos) MoveResult {
    const line_idx = pos.line;
    const col_idx = &pos.col;
    const max_col_limit = array_of_lines.items[0].len;

    while (col_idx.* < max_col_limit) {
        const next_col = col_idx.* + 1;
        if (next_col >= max_col_limit) return MoveResult.OutOfBounds;
        if (array_of_lines.items[line_idx][next_col] != MoveConst.Obstacle) {
            array_of_lines.items[line_idx][col_idx.*] = MoveConst.Visited;
            col_idx.* = next_col;
            array_of_lines.items[line_idx][col_idx.*] = MoveConst.Guard;
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveLeft(array_of_lines: *lines, pos: *GuardPos) MoveResult {
    const line_idx = pos.line;
    var col_idx: i64 = @intCast(pos.col);
    const max_col_limit = array_of_lines.items[0].len;

    while (col_idx < max_col_limit and col_idx >= 0) {
        const next_col = col_idx - 1;
        if (next_col >= max_col_limit or next_col < 0) return MoveResult.OutOfBounds;
        if (array_of_lines.items[line_idx][@intCast(next_col)] != MoveConst.Obstacle) {
            array_of_lines.items[line_idx][@intCast(col_idx)] = MoveConst.Visited;
            col_idx = next_col;
            pos.col = @intCast(col_idx);
            array_of_lines.items[line_idx][@intCast(col_idx)] = MoveConst.Guard;
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveDown(array_of_lines: *lines, pos: *GuardPos) MoveResult {
    const line_idx = &pos.line;
    const col_idx = pos.col;
    const max_line_limit = array_of_lines.items.len;

    while (line_idx.* < max_line_limit) {
        const next_line = line_idx.* + 1;
        if (next_line >= max_line_limit) return MoveResult.OutOfBounds;
        if (array_of_lines.items[next_line][col_idx] != MoveConst.Obstacle) {
            array_of_lines.items[line_idx.*][col_idx] = MoveConst.Visited;
            line_idx.* = next_line;
            array_of_lines.items[next_line][col_idx] = MoveConst.Guard;
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveUp(array_of_lines: *lines, pos: *GuardPos) MoveResult {
    var line_idx: i64 = @intCast(pos.line);
    const col_idx = pos.col;
    const max_line_limit = array_of_lines.items.len;

    while (line_idx < max_line_limit and line_idx >= 0) {
        const next_line = line_idx - 1;
        if (next_line >= max_line_limit or next_line < 0) return MoveResult.OutOfBounds;
        if (array_of_lines.items[@intCast(next_line)][col_idx] != MoveConst.Obstacle) {
            array_of_lines.items[@intCast(line_idx)][col_idx] = MoveConst.Visited;
            line_idx = @intCast(next_line);
            pos.line = @intCast(line_idx);
            array_of_lines.items[@intCast(next_line)][col_idx] = MoveConst.Guard;
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveSoldier(array_of_lines: *lines, pos: *GuardPos, direction: Direction) MoveResult {
    const line_idx = pos.line;
    const col_idx = pos.col;

    const max_line_limit = array_of_lines.items.len;
    if (max_line_limit <= 0 or line_idx >= max_line_limit) return MoveResult.OutOfBounds;
    const max_col_limit = array_of_lines.items[0].len;
    if (max_col_limit <= 0 or col_idx >= max_col_limit) return MoveResult.OutOfBounds;

    switch (direction) {
        .Up => return MoveUp(array_of_lines, pos),
        .Down => return MoveDown(array_of_lines, pos),
        .Left => return MoveLeft(array_of_lines, pos),
        .Right => return MoveRight(array_of_lines, pos),
    }
}

fn RotateRight(direction: Direction) Direction {
    switch (direction) {
        .Up => return .Right,
        .Down => return .Left,
        .Left => return .Up,
        .Right => return .Down,
    }
}

fn DistinctPosVisited(buffer: *const string) u64 {
    const pattern_to_find = [_]u8{MoveConst.Visited};
    return std.mem.count(u8, buffer.items, pattern_to_find[0..]) + 1;
}

pub fn main() !void {
    const fstream = try cwd.openFile("./example.txt", .{});
    defer fstream.close();

    const metadata = try fstream.metadata();
    const size_of_file = metadata.size();

    var buffer = string.fromOwnedSlice(allocator, try fstream.readToEndAlloc(allocator, size_of_file));
    defer buffer.deinit();

    var array_of_lines: lines = lines.fromOwnedSlice(allocator, try splitByLine.convertToSliceOfLines(allocator, &buffer));
    defer array_of_lines.deinit();

    var pos = findPosOfGuard(&array_of_lines).?;
    var direction = Direction.Up;
    while (MoveSoldier(&array_of_lines, &pos, direction) != .OutOfBounds) {
        std.log.debug("Soldier collided at line: {}, col: {}", pos);
        direction = RotateRight(direction);
    } else {
        std.log.debug("Soldier went OOB at line: {}, col: {}", pos);
    }

    const distinct_pos_visited = DistinctPosVisited(&buffer);
    std.log.debug("\n{s}\n", .{buffer.items});
    std.log.debug("======================================================", .{});
    std.log.debug("no of distinct positions visited: {}", .{distinct_pos_visited});
}
