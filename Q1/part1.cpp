// find the total distance between the left list and the right list,
// add up the distances between all of the pairs you found.

#include <algorithm>
#include <fstream>
#include <iterator>
#include <print>
#include <vector>

int64_t mod(int val)
{
    if (val < 0)
        return (0 - val);
    return val;
}

int main(int argc, char *argv[])
{
    std::ifstream f("./input.txt");

    std::vector<int> l1;
    std::vector<int> l2;
    for (std::istream_iterator<int> itrBegin{f}, itrEnd; itrBegin != itrEnd;)
    {
        l1.push_back(*itrBegin);
        ++itrBegin;

        if (itrBegin == itrEnd)
            break;

        l2.push_back(*itrBegin);
        ++itrBegin;
    }

    std::sort(l1.begin(), l1.end());
    std::sort(l2.begin(), l2.end());

    int64_t sum = 0;
    for (auto itrL1Begin = l1.begin(), itrL1End = l1.end(), itrL2Begin = l2.begin(),
              itrL2End = l2.end();
         (itrL1Begin != itrL1End) && (itrL2Begin != itrL2End); (++itrL1Begin, ++itrL2Begin))
    {
        sum += mod(*itrL1Begin - *itrL2Begin);
    }

    std::println("sum is {}", sum);
    return 0;
}
