type Credits = Credit[];

interface Credit {
    id?: string,
    username: string,
    contributions: string[]
}

const credits: Credits = [
  {
    id: '312715611413413889',
    username: 'y21',
    contributions: [
      'Wrote the fake eval command implementation',
      'Wrote/is writing [Cast](https://github.com/AssystDev/Cast) and [ASM](https://github.com/AssystDev/ASM)',
      'Wrote the BadTranslator REST functions that BadTranslator and OCRBadTranslator use',
      'Developed the [zx8 web scraper](https://zx8.jacher.io)'
    ]
  },
  {
    id: '687945863053443190',
    username: 'matmen',
    contributions: [
      'Wrote the [fAPI image maniplation API](https://fapi.dreadful.tech/) that several commands utilise via [fClient](https://github.com/Jacherr/fClient)'
    ]
  }
];

export default credits;
