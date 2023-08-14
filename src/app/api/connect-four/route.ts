import {NextRequest, NextResponse} from "next/server";
import {analyze_position} from "@johan-stph/connect-four-solver";
export const runtime = 'nodejs'

export async function GET(request: NextRequest) {
    if (request.method !== "GET") {
        return NextResponse.error()
    }
    // Extract the 'position' parameter from the request's query string
    const position = request.nextUrl.searchParams.get("position") || "";

    // Pass the extracted position to the analyze_position function
    const result = Array.from(analyze_position(position));
    return NextResponse.json(
        result
    )
}
