import input from './input'
import inputstring from './input'


export default function day1() {
  const nums = inputstring.split('\n').map(n => parseInt(n))
  const one = partOne(nums)
  const two = partTwo(nums)
}

function partOne(numsInput) {
  const nums = [...numsInput]
  for(const index in nums) {
    const current = nums.pop()
    const match = nums.find(num =>  num + current === 2020)
    if (match) {
      console.log(current * match)
      return true
    }
  }
}

function partTwo(numsInput) {
  const nums = [...numsInput]
  for(const index in nums) {
    const first = nums.pop()
    nums.find(second => {
      const third = nums.find(third => (first + second + third) === 2020)
      if (third) {
        console.log(first * second * third)
        return true
      }
    })
  }
}