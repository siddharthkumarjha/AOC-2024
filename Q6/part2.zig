const std = @import("std");
const TraceSoldierPath = @import("./TraceSoldierPath.zig");

const string = TraceSoldierPath.string;
const lines = TraceSoldierPath.lines;

const Direction = TraceSoldierPath.Direction;
const GuardPos = TraceSoldierPath.GuardPos;
const MoveResult = TraceSoldierPath.MoveResult;
const HashSet = std.hash_map.AutoHashMap(AreaPos, void);

const LoopConst = struct {
    usingnamespace TraceSoldierPath.MoveConst;
    const Loop: u8 = '0';
};
const AreaPos = struct {
    pos: GuardPos,
    direction: Direction,
};

const allocator = std.heap.page_allocator;
const cwd = std.fs.cwd();

fn MoveRight(array_of_lines: *lines, pos: *GuardPos, set: *HashSet) !MoveResult {
    const line_idx = pos.line;
    const col_idx = &pos.col;
    const max_col_limit = array_of_lines.items[0].len;
    const direction = .Right;

    while (col_idx.* < max_col_limit) {
        const next_col = col_idx.* + 1;
        if (next_col >= max_col_limit) return MoveResult.OutOfBounds;
        if (array_of_lines.items[line_idx][next_col] != LoopConst.Obstacle) {
            _ = try set.getOrPut(AreaPos{ .pos = pos.*, .direction = direction });
            const loop = doesPathLoop(pos, set, direction);
            col_idx.* = next_col;
            if (loop) {
                array_of_lines.items[line_idx][col_idx.*] = LoopConst.Loop;
            }
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveLeft(array_of_lines: *lines, pos: *GuardPos, set: *HashSet) !MoveResult {
    const line_idx = pos.line;
    var col_idx: i64 = @intCast(pos.col);
    const max_col_limit = array_of_lines.items[0].len;
    const direction = .Left;

    while (col_idx < max_col_limit and col_idx >= 0) {
        const next_col = col_idx - 1;
        if (next_col >= max_col_limit or next_col < 0) return MoveResult.OutOfBounds;
        if (array_of_lines.items[line_idx][@intCast(next_col)] != LoopConst.Obstacle) {
            _ = try set.getOrPut(AreaPos{ .pos = pos.*, .direction = direction });
            const loop = doesPathLoop(pos, set, direction);
            col_idx = next_col;
            pos.col = @intCast(col_idx);
            if (loop) {
                array_of_lines.items[line_idx][@intCast(col_idx)] = LoopConst.Loop;
            }
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveDown(array_of_lines: *lines, pos: *GuardPos, set: *HashSet) !MoveResult {
    const line_idx = &pos.line;
    const col_idx = pos.col;
    const max_line_limit = array_of_lines.items.len;
    const direction = .Down;

    while (line_idx.* < max_line_limit) {
        const next_line = line_idx.* + 1;
        if (next_line >= max_line_limit) return MoveResult.OutOfBounds;
        if (array_of_lines.items[next_line][col_idx] != LoopConst.Obstacle) {
            _ = try set.getOrPut(AreaPos{ .pos = pos.*, .direction = direction });
            const loop = doesPathLoop(pos, set, direction);
            line_idx.* = next_line;
            if (loop) {
                array_of_lines.items[next_line][col_idx] = LoopConst.Loop;
            }
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn MoveUp(array_of_lines: *lines, pos: *GuardPos, set: *HashSet) !MoveResult {
    var line_idx: i64 = @intCast(pos.line);
    const col_idx = pos.col;
    const max_line_limit = array_of_lines.items.len;
    const direction = .Up;

    while (line_idx < max_line_limit and line_idx >= 0) {
        const next_line = line_idx - 1;
        if (next_line >= max_line_limit or next_line < 0) return MoveResult.OutOfBounds;
        if (array_of_lines.items[@intCast(next_line)][col_idx] != LoopConst.Obstacle) {
            _ = try set.getOrPut(AreaPos{ .pos = pos.*, .direction = direction });
            const loop = doesPathLoop(pos, set, direction);
            line_idx = @intCast(next_line);
            pos.line = @intCast(line_idx);
            if (loop) {
                array_of_lines.items[@intCast(next_line)][col_idx] = LoopConst.Loop;
            }
        } else {
            return MoveResult.ObstacleCollision;
        }
    } else {
        return MoveResult.OutOfBounds;
    }
}

fn doesPathLoop(pos: *const GuardPos, set: *HashSet, direction: Direction) bool {
    const new_direction = TraceSoldierPath.RotateRight(direction);
    const path_visited = set.contains(AreaPos{ .pos = pos.*, .direction = new_direction });
    if (path_visited) {
        return true;
    } else {
        return false;
    }
}

fn MoveSoldier(array_of_lines: *lines, pos: *GuardPos, set: *HashSet, direction: Direction) !MoveResult {
    const line_idx = pos.line;
    const col_idx = pos.col;

    const max_line_limit = array_of_lines.items.len;
    if (max_line_limit <= 0 or line_idx >= max_line_limit) return MoveResult.OutOfBounds;
    const max_col_limit = array_of_lines.items[0].len;
    if (max_col_limit <= 0 or col_idx >= max_col_limit) return MoveResult.OutOfBounds;

    switch (direction) {
        .Up => return MoveUp(array_of_lines, pos, set),
        .Down => return MoveDown(array_of_lines, pos, set),
        .Left => return MoveLeft(array_of_lines, pos, set),
        .Right => return MoveRight(array_of_lines, pos, set),
    }
}

fn TraceLoopsInPath(buffer: *string, array_of_lines: *lines, set: *HashSet) !usize {
    const inital_pos = TraceSoldierPath.findPosOfGuard(array_of_lines).?;
    var pos = inital_pos;
    var direction = Direction.Up;
    while (try MoveSoldier(array_of_lines, &pos, set, direction) != .OutOfBounds) {
        std.log.debug("Soldier collided at line: {}, col: {}", pos);
        direction = TraceSoldierPath.RotateRight(direction);
    } else {
        std.log.debug("Soldier went OOB at line: {}, col: {}", pos);
        array_of_lines.items[inital_pos.line][inital_pos.col] = LoopConst.Guard;
        array_of_lines.items[pos.line][pos.col] = LoopConst.Visited;
    }

    const pattern_to_find = [_]u8{LoopConst.Loop};
    return std.mem.count(u8, buffer.items, pattern_to_find[0..]);
}

pub fn main() !void {
    var buffer = string.init(allocator);
    defer buffer.deinit();

    var array_of_lines = lines.init(allocator);
    defer array_of_lines.deinit();

    const distinct_pos_visited = try TraceSoldierPath.TraceSoldierPath(&buffer, &array_of_lines);
    std.log.debug("======================================================", .{});
    std.log.debug("no of distinct positions visited: {}", .{distinct_pos_visited});

    var visited_before = HashSet.init(allocator);
    defer visited_before.deinit();

    try visited_before.ensureUnusedCapacity(@intCast(distinct_pos_visited));

    const no_of_loops = try TraceLoopsInPath(&buffer, &array_of_lines, &visited_before);
    std.log.debug("\n{s}\n", .{buffer.items});
    std.log.debug("loop count: {}", .{no_of_loops});
}
