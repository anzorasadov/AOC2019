import inputstring from './input'


export default function day1() {
  const passwords = inputstring.split('\n').map(mapPasswordEntry)
  console.log(passwords.length)
  partOne([...passwords])
  partTwo([...passwords])
}

function partOne(passwords) {
  const validPasswords = []

  passwords.forEach(p => {
    const letterCount = p.password.split('').filter(char => char === p.letter).length
    if (letterCount >= p.min && letterCount <= p.max) {
      validPasswords.push(p)
    }
  })

  console.log('part one: ', validPasswords.length)
}

function partTwo(passwords) {
  const validPasswords = []

  passwords.forEach(p => {
    const allLetters = p.password.split('')
    if(allLetters[p.min - 1] == p.letter ^ allLetters[p.max - 1] == p.letter){
      validPasswords.push(p)
    }
  })

  console.log('part two: ', validPasswords.length)
}

function mapPasswordEntry(rawEntry) {
  const passEntry = rawEntry.split(/[ \-\:]/).filter(i => i)
  return { min: passEntry[0], max: passEntry[1], letter: passEntry[2], password: passEntry[3] }
}