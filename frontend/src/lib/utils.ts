export function calculateReadingTime(article: { word_count: number; image_count: number; heading_count: number }): string {
	const WPM = 238;
	let imageSeconds = 0;
	
	const imageCount = article.image_count || 0;
	for (let i = 1; i <= imageCount; i++) {
		imageSeconds += Math.max(3, 12 - (i - 1));
	}

	const proseMinutes = (article.word_count || 0) / WPM;
	const headingSeconds = (article.heading_count || 0) * 1.5;

	const totalSeconds = proseMinutes * 60 + imageSeconds + headingSeconds;
	const totalMinutes = Math.max(1, Math.ceil(totalSeconds / 60));

	if (totalMinutes >= 30) {
		return `${totalMinutes}–${totalMinutes + 5} min read`;
	}
	return `${totalMinutes} min read`;
}
