# RowdyHacks X 1st Place Overall Winner!
https://devpost.com/software/rowdypay

## Inspiration
Have you ever gone out to eat with the same friends only to have to coordinate how to split the bill properly for the seven hundredth time? It's such a pain. Who pays who? How much do I owe you? How should I pay you? Therefore, we created RowdyPay, a mobile budgeting solution that seeks to end this inconvenience once and for all.

## What it does
RowdyPay is an iOS app that allows groups of users to manage shared budgets. Users can scan receipts and utilize Optical Character Recognition to parse the details. From there, users can automatically send a payment request to a shared budget, where other users in the group can then contribute their share.

## How we built it
RowdyPay uses SwiftUI for a mobile iOS frontend, which connects via a REST API to a blazingly fast backend written entirely in Rust. We used one of our team's laptops as a server and took advantage of the OpenAI API to perform OCR on pictures of receipts.

## Challenges we ran into
Early on, we knew it was imperative that we had the proper setup for the rest of the project. We had to spend meticulous effort in setting up the middleware correctly the first time around, which meant there were more than a few refactorings.

## Accomplishments that we're proud of
We are incredibly proud of the functionality of our app. It's a working product, done right! We have a beautiful front end paired with a robust and multithreaded backend, with no loose screws left anywhere.

## What we learned
We learned that communication is absolutely essential, especially when working in projects that have many moving parts. Several times we had to pause our work and orient ourselves in order to keep moving in the right direction.

## What's next for RowdyPay
We hope that we can polish RowdyPay into a mobile app worthy of the Apple App Store!
