export interface IBook {
    id: string;
    name: string;
    author: string;
    link: URL; // B&N link
    review: string;
    fiction: boolean;
    genre: BookGenre;
    series: boolean;
    bookLength: BookLength;
}

export enum BookGenre {
    // Fiction
    Fantasy = "fantasy",
    SciFi = "sci-fi",
    Dystopia = "dystopian",
    Action = "action",
    Mystery = "mystery",

    // Non-fiction
    Math = "math",
}

export enum BookLength {
    Small = "small",
    Medium = "medium-sized",
    Large = "large",
}

export interface IAnswer {
    fiction?: boolean,
    genre?: BookGenre,
    series?: boolean,
    bookLength?: BookLength,
}

export function suggestBooks(answer: IAnswer): {books: IBook[], hits: number} {
    // Return if any value is null
    if (answer.fiction === null || answer.genre === null || answer.series === null || answer.bookLength === null) {
        return {
            books: [],
            hits: -99999,
        };
    }

    let book_hits: {book: IBook, hits: number}[] = [];

    // Calculate all hits and misses
    for (const book of BOOKS) {
        let hits = 0;

        if (answer.fiction === book.fiction) {
            hits += 1;
        } else {
            hits -= 1;
        }

        if (answer.genre === book.genre) {
            hits += 1;
        } else {
            hits -= 1;
        }

        if (answer.series === book.series) {
            hits += 1;
        } else {
            hits -= 1;
        }

        if (answer.bookLength === book.bookLength) {
            hits += 1;
        } else {
            hits -= 1;
        }

        book_hits.push({
            book: book,
            hits: hits,
        });
    }

    // Find books with most hits
    let best_books: {books: IBook[], hits: number} = {
        books: [],
        hits: -99999, // Make it so anything is better than nothing
    };

    for (const book_data of book_hits) {
        if (book_data.hits > best_books.hits) {
            // If better hit, replace everything else
            best_books = {
                books: [book_data.book],
                hits: book_data.hits,
            };
        } else if (book_data.hits === best_books.hits) {
            // If hit is the same, add it to list
            best_books.books.push(book_data.book);
        }
    }

    return best_books;
}

const BNBase = "https://www.barnesandnoble.com/w/";

export const BOOKS: IBook[] = [
    {
        id: "the-way-of-kings",
        name: "The Way of Kings",
        author: "Brandon Sanderson",
        link: new URL("way-of-kings-brandon-sanderson/1100354496", BNBase),
        review: "A 3-inch thick book, but one of the best series I've ever read.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: true,
        bookLength: BookLength.Large,
    },
    {
        id: "mistborn",
        name: "Mistborn",
        author: "Brandon Sanderson",
        link: new URL("mistborn-brandon-sanderson/1100833141", BNBase),
        review: "If you want the same quality as The Way of Kings, but was intimidated by its size, this book is for you.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: true,
        bookLength: BookLength.Medium,
    },
    {
        id: "elantris",
        name: "Elantris",
        author: "Brandon Sanderson",
        link: new URL("elantris-brandon-sanderson/1100356144", BNBase),
        review: "This is the first book written by Brandon Sanderson. While the beginning is choppy, the story and worldbuilding make up for it.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: false,
        bookLength: BookLength.Medium,
    },
    {
        id: "warbreaker",
        name: "Warbreaker",
        author: "Brandon Sanderson",
        link: new URL("warbreaker-brandon-sanderson/1014955753", BNBase),
        review: "An amazing standalone novel that incorporates color and magic. It has multiple protagonists, and brings the story together quite nicely.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: false,
        bookLength: BookLength.Medium,
    },
    {
        id: "wings-of-fire",
        name: "Wings of Fire",
        author: "Tui T. Sutherland",
        link: new URL("the-dragonet-prophecy-tui-t-sutherland/1108166874", BNBase),
        review: "I started reading this 6 years ago, and have been ever since. There are many books in this series, each story arc containing 5 books.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: true,
        bookLength: BookLength.Medium,
    },
    {
        id: "renegades",
        name: "Renegades",
        author: "Marissa Meyer",
        link: new URL("renegades-marissa-meyer/1125840006", BNBase),
        review: "A friend recommended this to me, and I was not left down. This is the world where superheros are real, but not always there to save you. It has quite an interesting perspective.",
        fiction: true,
        genre: BookGenre.Dystopia,
        series: true,
        bookLength: BookLength.Large,
    },
    {
        id: "how-not-to-be-wrong",
        name: "How Not to Be Wrong",
        author: "Jordan Ellenberg",
        link: new URL("how-not-to-be-wrong-jordan-ellenberg/1117225089", BNBase),
        review: "I found this book in Barnes & Noble. It's a bit highschool math heavy, but presents some interesting ways of thinking.",
        fiction: false,
        genre: BookGenre.Math,
        series: false,
        bookLength: BookLength.Medium,
    },
    {
        id: "shadow-of-the-conqueror",
        name: "Shadow of the Conqueror",
        author: "Shad M. Brooks",
        link: new URL("shadow-of-the-conqueror-shad-m-brooks/1132130546", BNBase),
        review: "This book was written by a person who runs a worldbuilding YouTube channel. He does not let down, though this is slightly more mature.",
        fiction: true,
        genre: BookGenre.Fantasy,
        series: false, // Technically yes, but no more books have been written yet
        bookLength: BookLength.Medium,
    },
    {
        id: "artemis-fowl",
        name: "Artemis Fowl",
        author: "Eoin Colfer",
        link: new URL("artemis-fowl-eoin-colfer/1100152822", BNBase),
        review: "An epic heist-themed book that incorporated fantasy at the same time. Quite a thrilling read.",
        fiction: true,
        genre: BookGenre.Mystery,
        series: true,
        bookLength: BookLength.Medium,
    },
];
