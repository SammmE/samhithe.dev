'use client';

import { useState, useEffect } from 'react';

interface TypewriterProps {
    text: string;
    speed?: number;
    delay?: number;
    className?: string;
}

const Typewriter = ({ text, speed = 50, delay = 0, className = '' }: TypewriterProps) => {
    const [displayText, setDisplayText] = useState('');
    const [started, setStarted] = useState(false);

    useEffect(() => {
        const timer = setTimeout(() => {
            setStarted(true);
        }, delay);
        return () => clearTimeout(timer);
    }, [delay]);

    useEffect(() => {
        if (!started) return;

        if (displayText.length < text.length) {
            const timeout = setTimeout(() => {
                setDisplayText(text.slice(0, displayText.length + 1));
            }, speed);
            return () => clearTimeout(timeout);
        }
    }, [displayText, started, speed, text]);

    return <span className={className}>{displayText}</span>;
};

export default Typewriter;
