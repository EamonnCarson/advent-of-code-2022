import numpy as np
import pandas as pd

with open("res/day08/input.real") as f:
    data = np.array([
        [int(c) for c in line.strip()]
        for line in f
    ])

print(data)


def visible_forward(heights):
    max_height = -1
    visible_front = []
    for h in heights:
        visible_front.append(h > max_height)
        max_height = max(h, max_height)
    return np.array(visible_front)

def visible(heights):
    heights = heights.reshape(-1)
    forwards = visible_forward(heights)
    backwards = visible_forward(heights[::-1])[::-1]
    return forwards | backwards

visible_north_south = np.stack(
    [visible(data[:, i]) for i in range(data.shape[1])],
    axis=1
)

visible_east_west = np.stack(
    [visible(data[i, :]) for i in range(data.shape[0])],
    axis=0
)

visible = visible_east_west | visible_north_south
print(visible)
print(np.sum(visible))


def scenic_forward(heights):
    scenic_scores = []
    for i, h in enumerate(heights):
        scenic_score = 0
        j = i - 1
        while j >= 0 and heights[j] < h:
            scenic_score += 1
            j -= 1
        if j >= 0: # we stopped early bc higher tree
            scenic_score += 1
        scenic_scores.append(scenic_score)
    return np.array(scenic_scores)

def scenic(heights):
    heights = heights.reshape(-1)
    forwards = scenic_forward(heights)
    backwards = scenic_forward(heights[::-1])[::-1]
    return forwards * backwards

scenic_north_south = np.stack(
    [scenic(data[:, i]) for i in range(data.shape[1])],
    axis=1
)

scenic_east_west = np.stack(
    [scenic(data[i, :]) for i in range(data.shape[0])],
    axis=0
)

scenic = scenic_east_west * scenic_north_south
print(scenic)
print(np.max(scenic))
