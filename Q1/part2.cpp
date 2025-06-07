// figure out exactly how often each number from the left list appears in the right list.
// Calculate a total similarity score by adding up each number in the left list after multiplying it
// by the number of times that number appears in the right list.

#include <fstream>
#include <iterator>
#include <map>
#include <print>
#include <vector>

int main(int argc, char *argv[])
{
    std::ifstream f("./input.txt");

    std::vector<int> leftList;
    std::map<int, int> freqMap;
    for (std::istream_iterator<int> itrBegin{f}, itrEnd; itrBegin != itrEnd;)
    {
        leftList.push_back(*itrBegin);
        ++itrBegin;

        if (itrBegin == itrEnd)
            break;

        freqMap[*itrBegin]++;
        ++itrBegin;
    }

    int64_t sum = 0;
    for(const auto &leftElem : leftList)
    {
        auto itrFreq = freqMap.find(leftElem);
        if (itrFreq != freqMap.end())
            sum += (leftElem * (itrFreq->second));
    }

    std::println("similarity score is {}", sum);
    return 0;
}
