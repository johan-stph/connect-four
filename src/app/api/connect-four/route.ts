import {NextRequest, NextResponse} from "next/server";
import {analyze_position} from "@johan-stph/connect-four-solver";

export async function GET(request: NextRequest) {
    // Extract the 'position' parameter from the request's query string
    const position = request.nextUrl.searchParams.get("position") || "";

    // Pass the extracted position to the analyze_position function
    const result = Array.from(analyze_position(position));
    return NextResponse.json(
        result
    )
}
