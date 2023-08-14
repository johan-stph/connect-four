import './globals.css'
import type {Metadata} from 'next'
import {Inter} from 'next/font/google'
import NavBar from "@/components/NavBar";
import React from "react";

import Favicon from '/public/favicon.ico';

const inter = Inter({subsets: ['latin']})

export const metadata: Metadata = {
    title: 'Connect 4',
    description: 'Connect 4 game',
    icons: [{rel: 'icon', url: Favicon.src}],
}

export default function RootLayout({
                                       children,
                                   }: {
    children: React.ReactNode
}) {
    return (
        <html lang="en">
            <body className={inter.className}>
                <NavBar></NavBar>
                {children}
            </body>
        </html>
    )
}
