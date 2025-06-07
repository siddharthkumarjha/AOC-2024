/*
 * Each report is a list of numbers called levels that are separated by spaces. For example:
 *
 * 7 6 4 2 1
 * 1 2 7 8 9
 * 9 7 6 2 1
 * 1 3 2 4 5
 * 8 6 4 4 1
 * 1 3 6 7 9
 *
 * This example data contains six reports each containing five levels.
 *
 * The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety
 * systems can only tolerate levels that are either gradually increasing or gradually decreasing.
 * So, a report only counts as safe if both of the following are true:
 *
 *     The levels are either all increasing or all decreasing.
 *     Any two adjacent levels differ by at least one and at most three.
 *
 * In the example above, the reports can be found safe or unsafe by checking those rules:
 *
 *     7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
 *     1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
 *     9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
 *     1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
 *     8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
 *     1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
 *
 * So, in this example, 2 reports are safe.
 */

#include <fstream>
#include <iterator>
#include <print>
#include <sstream>
#include <string>
#include <vector>

int mod(int const val)
{
    if (val < 0)
        return (0 - val);
    return val;
}

bool checkRules(int const &a, int const &b)
{
    int diff = mod(a - b);
    if (diff >= 1 && diff <= 3)
        return true;
    return false;
}

bool checkStrictlyDecreasing(std::vector<int> &levels)
{
    if (levels.empty())
        return false;

    auto itrBegin     = levels.begin();
    auto const itrEnd = levels.end();

    auto prevLevel = *itrBegin;
    ++itrBegin;
    while (itrBegin != itrEnd)
    {
        const int &thisLevel = *itrBegin;
        if ((thisLevel > prevLevel) && checkRules(thisLevel, prevLevel))
        {
            prevLevel = thisLevel;
            ++itrBegin;
        }
        else
        {
            return false;
        }
    }
    return true;
}

bool checkStrictlyIncreasing(std::vector<int> &levels)
{
    if (levels.empty())
        return false;

    auto itrBegin     = levels.begin();
    auto const itrEnd = levels.end();

    auto prevLevel = *itrBegin;
    ++itrBegin;
    while (itrBegin != itrEnd)
    {
        const int &thisLevel = *itrBegin;
        if ((thisLevel < prevLevel) && checkRules(thisLevel, prevLevel))
        {
            prevLevel = thisLevel;
            ++itrBegin;
        }
        else
        {
            return false;
        }
    }
    return true;
}

int main(int argc, char *argv[])
{
    std::ifstream f("./input.txt");

    std::string line;
    int safeCount = 0;
    while (std::getline(f, line))
    {
        std::istringstream strm(std::move(line));
        std::vector<int> levels{std::istream_iterator<int>{strm}, std::istream_iterator<int>{}};

        bool isStrictlyIncreasing = checkStrictlyIncreasing(levels);
        bool isStrictlyDecreasing = checkStrictlyDecreasing(levels);
        if ((true == isStrictlyIncreasing) || (true == isStrictlyDecreasing))
        {
            ++safeCount;
        }
    }
    std::println("{} reports are safe", safeCount);
    return 0;
}
