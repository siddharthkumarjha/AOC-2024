// add tolerance that if there is only 1 error in the report of levels.
// And if removing this level from the report fixes the report and makes it safe
// It can be marked safe

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

auto checkStrictlyDecreasing(std::vector<int> &levels)
    -> std::pair<bool, std::vector<int>::const_iterator>
{
    if (levels.empty())
        return {false, levels.cend()};

    auto itrBegin     = levels.cbegin();
    auto const itrEnd = levels.cend();

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
            return {false, itrBegin};
        }
    }
    return {true, itrEnd};
}

auto checkStrictlyIncreasing(std::vector<int> const &levels)
    -> std::pair<bool, std::vector<int>::const_iterator>
{
    if (levels.empty())
        return {false, levels.cend()};

    auto itrBegin     = levels.cbegin();
    auto const itrEnd = levels.cend();

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
            return {false, itrBegin};
        }
    }
    return {true, itrEnd};
}

template <typename FuncTor>
bool recheckWithOneElemRemoved(FuncTor fnCheck, std::vector<int>::const_iterator itrViolation,
                               std::vector<int> const &levels)
{
    int violationIdx = itrViolation - levels.begin();
    if (violationIdx < 0 || violationIdx >= levels.size())
        return false;

    // Candidates for removal:
    std::vector<size_t> candidates;
    candidates.reserve(3);

    // Current violation index
    candidates.push_back(violationIdx);

    // Next element if it exists
    if ((violationIdx + 1) < levels.size())
        candidates.push_back(violationIdx + 1);

    // Previous element if it exists
    if (violationIdx > 0)
        candidates.push_back(violationIdx - 1);

    for (const auto &idx : candidates)
    {
        auto tmp = levels;
        tmp.erase(tmp.begin() + idx);
        auto const [funcTorStatus, _] = fnCheck(tmp);

        if (funcTorStatus)
            return true;
    }

    return false;
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

        auto const [isStrictlyIncreasing, itrStrictlyIncViolation] =
            checkStrictlyIncreasing(levels);
        auto const [isStrictlyDecreasing, itrStrictlyDecViolation] =
            checkStrictlyDecreasing(levels);

        if ((true == isStrictlyIncreasing) || (true == isStrictlyDecreasing))
        {
            ++safeCount;
        }
        else if (!levels.empty())
        {
            bool safeWithOneRemoval = false;
            safeWithOneRemoval =
                recheckWithOneElemRemoved(checkStrictlyIncreasing, itrStrictlyIncViolation, levels);
            if (!safeWithOneRemoval)
                safeWithOneRemoval = recheckWithOneElemRemoved(checkStrictlyDecreasing,
                                                               itrStrictlyDecViolation, levels);
            if (safeWithOneRemoval)
                ++safeCount;
        }
    }

    std::println("{} reports are safe", safeCount);
    return 0;
}
