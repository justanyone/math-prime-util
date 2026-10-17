Port the four twin prime functions from MPU 0.75 with the C bounds and tables.
Keep each function in its own families file and use existing prime helpers.
Use the committed families fixtures without changing tools or fixture data.
Run focused offline tests, then the required gate. Leave git and STATUS.md unchanged.
Read the C wheel scan, count tables, approximation corrections, and all four fixture input ranges. Implement a bounded segmented wheel scan to share between listing, counting, and selection.
