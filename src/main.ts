import './style.css';

const canvas = document.getElementById('stars') as HTMLCanvasElement;
const context = canvas.getContext('2d');
if (!context) throw new Error('Canvas context not available');
const ctx: CanvasRenderingContext2D = context;

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
let width = canvas.width;
let height = canvas.height;

interface Star {
    x: number;
    y: number;
    radius: number;
    vx: number;
    vy: number;
    opacity: number;
    hue?: number;
    mass?: number;
    depth: number;
    baseY: number;
}

interface Particle {
    x: number;
    y: number;
    vx: number;
    vy: number;
    radius: number;
    life: number;
    maxLife: number;
    hue: number;
}

const stars: Star[] = [];
const particles: Particle[] = [];
const starCount = 500;
let gravityEnabled = false;
let connectionsEnabled = true;
let attractionMode = false;
let repulsionMode = false;
let trailsEnabled = false;
let warpSpeedEnabled = false;
let colorShiftEnabled = false;
let blackHoleEnabled = false;
let mouseParallaxEnabled = false;
const gravitationalConstant = 0.5;
const minDistance = 5;
const maxConnectionDistance = 150;
const maxConnections = 10;
const mouseForceStrength = 0.8;
const mouseForceRadius = 200;
const warpSpeedMultiplier = 20;
let blackHoleMass = 5000;
let eventHorizonRadius = 30;
const accretionDiskRadius = 200;
const schwarzschildConstant = 2;
const speedOfLight = 10;
const PARALLAX_SPEED = 0.5;
const MOUSE_PARALLAX_SPEED = 0.02;

const mouse = { x: width / 2, y: height / 2 };

let bigBangActive = true;
let bigBangTime = 0;
const bigBangDuration = 240;
const bigBangCenter = { x: width / 2, y: height / 2 };

let currentSection = 0;
const sections = ['landing', 'about', 'projects', 'contact'];
let isTransitioning = false;
let targetScrollY = 0;
const scrollSmoothing = 0.05;
let scrollVelocity = 0;
const velocityDamping = 0.92;

const controlElements = new Map<string, Element>();
['c', 'g', 'a', 'r', 't', 'w', 'x', 'b', 'm'].forEach(key => {
    const element = document.querySelector(`[data-key="${key}"]`);
    if (element) controlElements.set(key, element);
});

function updateControlUI() {
    const controls = {
        'c': connectionsEnabled,
        'g': gravityEnabled,
        'a': attractionMode,
        'r': repulsionMode,
        't': trailsEnabled,
        'w': warpSpeedEnabled,
        'x': colorShiftEnabled,
        'b': blackHoleEnabled,
        'm': mouseParallaxEnabled
    };

    Object.entries(controls).forEach(([key, enabled]) => {
        const element = controlElements.get(key);
        if (element) {
            element.classList.toggle('active', enabled);
        }
    });
}

for (let i = 0; i < starCount; i++) {
    const radius = Math.random() * 1.5 + 0.5;
    const angle = Math.random() * Math.PI * 2;
    const speed = (Math.random() * 0.8 + 0.4) * 3.5;
    const depth = Math.random();

    stars.push({
        x: bigBangCenter.x,
        y: bigBangCenter.y,
        radius: radius,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        opacity: 0,
        hue: Math.random() * 360,
        mass: radius * radius * Math.PI,
        depth: depth,
        baseY: bigBangCenter.y
    });
}

function animate() {
    if (bigBangActive) {
        bigBangTime++;

        ctx.fillStyle = 'rgba(10, 10, 10, 0.15)';
        ctx.fillRect(0, 0, width, height);

        const progress = bigBangTime / bigBangDuration;
        const fadeInPhase = Math.min(Math.max((progress - 0.2) * 1.5, 0), 1);

        if (bigBangTime < 80) {
            const flashIntensity = Math.max(0, 1 - bigBangTime / 80);
            const flashRadius = bigBangTime * 15;

            const gradient = ctx.createRadialGradient(
                bigBangCenter.x, bigBangCenter.y, 0,
                bigBangCenter.x, bigBangCenter.y, flashRadius
            );
            gradient.addColorStop(0, `rgba(255, 255, 255, ${flashIntensity})`);
            gradient.addColorStop(0.3, `rgba(255, 200, 100, ${flashIntensity * 0.7})`);
            gradient.addColorStop(0.6, `rgba(255, 100, 50, ${flashIntensity * 0.4})`);
            gradient.addColorStop(1, 'rgba(255, 0, 0, 0)');

            ctx.beginPath();
            ctx.arc(bigBangCenter.x, bigBangCenter.y, flashRadius, 0, Math.PI * 2);
            ctx.fillStyle = gradient;
            ctx.fill();
        }

        stars.forEach((star) => {
            const explosionCurve = (1 - progress) ** 2;
            const dragFactor = 0.985 + progress * 0.014;

            star.vx *= dragFactor;
            star.vy *= dragFactor;

            const movementMultiplier = 1 + explosionCurve * 2;
            star.x += star.vx * movementMultiplier;
            star.y += star.vy * movementMultiplier;

            if (star.x < 0) star.x = width;
            if (star.x > width) star.x = 0;
            if (star.y < 0) star.y = height;
            if (star.y > height) star.y = 0;

            const targetOpacity = Math.random() * 0.5 + 0.5;
            star.opacity = star.opacity * 0.95 + fadeInPhase * targetOpacity * 0.05;

            if (colorShiftEnabled || bigBangTime < bigBangDuration * 0.85) {
                star.hue = ((star.hue || 0) + 0.8) % 360;
            }

            ctx.beginPath();
            ctx.arc(star.x, star.y, star.radius, 0, Math.PI * 2);
            ctx.fillStyle = `hsla(${star.hue}, 80%, 70%, ${star.opacity})`;
            ctx.fill();

            if (bigBangTime < 120) {
                const trailFade = Math.max(0, 1 - bigBangTime / 120);
                const trailLength = trailFade * 80;

                ctx.beginPath();
                ctx.moveTo(star.x - star.vx * trailLength, star.y - star.vy * trailLength);
                ctx.lineTo(star.x, star.y);

                const gradient = ctx.createLinearGradient(
                    star.x - star.vx * trailLength,
                    star.y - star.vy * trailLength,
                    star.x,
                    star.y
                );
                gradient.addColorStop(0, `hsla(${star.hue}, 80%, 70%, 0)`);
                gradient.addColorStop(1, `hsla(${star.hue}, 80%, 70%, ${star.opacity * trailFade})`);

                ctx.strokeStyle = gradient;
                ctx.lineWidth = star.radius * 2;
                ctx.stroke();
            }
        });

        if (bigBangTime >= bigBangDuration) {
            bigBangActive = false;
            if (!colorShiftEnabled) {
                stars.forEach(star => {
                    star.hue = undefined;
                });
            }
        }

        requestAnimationFrame(animate);
        return;
    }

    if (trailsEnabled) {
        ctx.fillStyle = 'rgba(10, 10, 10, 0.15)';
        ctx.fillRect(0, 0, width, height);
    } else {
        ctx.clearRect(0, 0, width, height);
    }

    const speedMultiplier = warpSpeedEnabled ? warpSpeedMultiplier : 1;

    if (gravityEnabled) {
        stars.forEach(star1 => {
            stars.forEach(star2 => {
                if (star1 === star2) return;

                const dx = star2.x - star1.x;
                const dy = star2.y - star1.y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance > minDistance) {
                    const force = (gravitationalConstant * star1.radius * star2.radius) / (distance * distance);
                    const fx = (dx / distance) * force;
                    const fy = (dy / distance) * force;

                    star1.vx += fx;
                    star1.vy += fy;
                }
            });
        });
    }

    if (attractionMode || repulsionMode) {
        stars.forEach(star => {
            const dx = mouse.x - star.x;
            const dy = mouse.y - star.y;
            const distance = Math.sqrt(dx * dx + dy * dy);

            if (distance < mouseForceRadius && distance > 1) {
                const force = mouseForceStrength / distance;
                const fx = (dx / distance) * force;
                const fy = (dy / distance) * force;

                if (attractionMode) {
                    star.vx += fx;
                    star.vy += fy;
                } else {
                    star.vx -= fx;
                    star.vy -= fy;
                }
            }
        });
    }

    if (blackHoleEnabled) {
        eventHorizonRadius = schwarzschildConstant * blackHoleMass / (speedOfLight * speedOfLight);
        if (eventHorizonRadius < 20) eventHorizonRadius = 20;
        if (eventHorizonRadius > 100) eventHorizonRadius = 100;

        const photonSphereRadius = eventHorizonRadius * 1.5;
        const innerStableOrbitRadius = eventHorizonRadius * 3;

        stars.forEach((star) => {
            const dx = mouse.x - star.x;
            const dy = mouse.y - star.y;
            const distance = Math.sqrt(dx * dx + dy * dy);

            if (distance < eventHorizonRadius) {
                blackHoleMass += star.mass || 1;

                const accretionParticles = 8;
                for (let i = 0; i < accretionParticles; i++) {
                    const angle = (Math.PI * 2 * i) / accretionParticles + Math.random();
                    particles.push({
                        x: star.x,
                        y: star.y,
                        vx: Math.cos(angle) * 2,
                        vy: Math.sin(angle) * 2,
                        radius: star.radius * 0.3,
                        life: 30,
                        maxLife: 30,
                        hue: star.hue || 280
                    });
                }

                const newRadius = Math.random() * 1.5 + 0.5;
                star.x = Math.random() * width;
                star.y = Math.random() * height;
                star.vx = (Math.random() - 0.5) * 0.3;
                star.vy = (Math.random() - 0.5) * 0.3;
                star.radius = newRadius;
                star.mass = newRadius * newRadius * Math.PI;
                return;
            }

            if (distance < accretionDiskRadius && distance > eventHorizonRadius) {
                const gravitationalForce = (blackHoleMass * (star.mass || 1)) / (distance * distance);
                const acceleration = gravitationalForce / (star.mass || 1);

                const fx = (dx / distance) * acceleration;
                const fy = (dy / distance) * acceleration;

                star.vx += fx * 0.008;
                star.vy += fy * 0.008;

                if (distance > photonSphereRadius) {
                    const tangentX = -dy / distance;
                    const tangentY = dx / distance;

                    const currentVelX = star.vx;
                    const currentVelY = star.vy;
                    const radialVelX = (dx / distance) * ((currentVelX * dx + currentVelY * dy) / distance);
                    const radialVelY = (dy / distance) * ((currentVelX * dx + currentVelY * dy) / distance);
                    const tangentialVelX = currentVelX - radialVelX;
                    const tangentialVelY = currentVelY - radialVelY;

                    const circularSpeed = Math.sqrt(blackHoleMass / distance);
                    const currentTangentialSpeed = Math.sqrt(tangentialVelX * tangentialVelX + tangentialVelY * tangentialVelY);
                    const tangentialCorrection = (circularSpeed - currentTangentialSpeed) * 0.05;

                    star.vx += tangentX * tangentialCorrection;
                    star.vy += tangentY * tangentialCorrection;
                }

                if (distance < innerStableOrbitRadius) {
                    const speed = Math.sqrt(star.vx * star.vx + star.vy * star.vy);
                    const escapeVelocity = Math.sqrt(2 * blackHoleMass / distance);

                    if (speed < escapeVelocity * 0.5) {
                        const spiralForce = 0.015;
                        star.vx += dx / distance * spiralForce;
                        star.vy += dy / distance * spiralForce;
                    }
                }

                const redshiftFactor = Math.sqrt(1 - (2 * blackHoleMass) / (distance * speedOfLight * speedOfLight));
                if (star.hue !== undefined && colorShiftEnabled) {
                    star.hue = (star.hue + (1 - redshiftFactor) * 2) % 360;
                }
            }
        });
    }

    if (connectionsEnabled) {
        const closestStars: Array<{ star: Star, distance: number }> = [];

        for (let i = 0; i < stars.length; i++) {
            const star = stars[i];
            const dx = star.x - mouse.x;
            const dy = star.y - mouse.y;
            const distance = Math.sqrt(dx * dx + dy * dy);

            if (distance < maxConnectionDistance) {
                if (closestStars.length < maxConnections) {
                    closestStars.push({ star, distance });
                    closestStars.sort((a, b) => a.distance - b.distance);
                } else if (distance < closestStars[maxConnections - 1].distance) {
                    closestStars[maxConnections - 1] = { star, distance };
                    closestStars.sort((a, b) => a.distance - b.distance);
                }
            }
        }

        closestStars.forEach(({ star, distance }) => {
            const opacity = 1 - (distance / maxConnectionDistance);
            ctx.beginPath();
            ctx.moveTo(mouse.x, mouse.y);
            ctx.lineTo(star.x, star.y);
            ctx.strokeStyle = `rgba(255, 255, 255, ${opacity * 0.5})`;
            ctx.lineWidth = 1;
            ctx.stroke();
        });
    }

    if (blackHoleEnabled) {
        const gradient = ctx.createRadialGradient(mouse.x, mouse.y, 0, mouse.x, mouse.y, accretionDiskRadius);
        gradient.addColorStop(0, 'rgba(0, 0, 0, 1)');
        gradient.addColorStop(eventHorizonRadius / accretionDiskRadius, 'rgba(30, 0, 60, 0.9)');
        gradient.addColorStop((eventHorizonRadius * 1.5) / accretionDiskRadius, 'rgba(80, 20, 120, 0.5)');
        gradient.addColorStop(0.6, 'rgba(150, 50, 200, 0.25)');
        gradient.addColorStop(0.85, 'rgba(200, 100, 255, 0.1)');
        gradient.addColorStop(1, 'rgba(255, 150, 255, 0)');

        ctx.beginPath();
        ctx.arc(mouse.x, mouse.y, accretionDiskRadius, 0, Math.PI * 2);
        ctx.fillStyle = gradient;
        ctx.fill();

        ctx.beginPath();
        ctx.arc(mouse.x, mouse.y, eventHorizonRadius * 1.5, 0, Math.PI * 2);
        ctx.strokeStyle = 'rgba(150, 100, 255, 0.4)';
        ctx.lineWidth = 1;
        ctx.setLineDash([5, 5]);
        ctx.stroke();
        ctx.setLineDash([]);

        ctx.beginPath();
        ctx.arc(mouse.x, mouse.y, eventHorizonRadius, 0, Math.PI * 2);
        ctx.fillStyle = 'rgba(0, 0, 0, 1)';
        ctx.fill();
        ctx.strokeStyle = `rgba(100, 50, 200, ${Math.min(blackHoleMass / 10000, 1)})`;
        ctx.lineWidth = 2;
        ctx.stroke();
    }

    const scrollDiff = window.scrollY - targetScrollY;
    scrollVelocity = scrollVelocity * velocityDamping + scrollDiff * (1 - velocityDamping);
    targetScrollY += scrollVelocity * scrollSmoothing;

    const time = Date.now() * 0.0002;

    stars.forEach((star) => {
        const depthCurve = star.depth ** 2.2;
        const invDepthCurve = 1 - depthCurve;
        const parallaxFactor = 0.005 + depthCurve * 0.12;
        const scrollOffset = -scrollVelocity * parallaxFactor * PARALLAX_SPEED;

        let parallaxX = 0;
        let parallaxY = 0;

        if (mouseParallaxEnabled) {
            const mouseDX = mouse.x - width / 2;
            const mouseDY = mouse.y - height / 2;
            parallaxX = mouseDX * invDepthCurve * MOUSE_PARALLAX_SPEED;
            parallaxY = mouseDY * invDepthCurve * MOUSE_PARALLAX_SPEED * 0.6;
        }

        const floatX = Math.sin(time + star.x * 0.01) * invDepthCurve * 0.2;
        const floatY = Math.cos(time + star.y * 0.01) * invDepthCurve * 0.2;

        star.x += (star.vx + parallaxX + floatX) * speedMultiplier;
        star.y += (star.vy + parallaxY + floatY + scrollOffset) * speedMultiplier;

        if (star.x < -100) star.x = width + 100;
        if (star.x > width + 100) star.x = -100;
        if (star.y < -100) star.y = height + 100;
        if (star.y > height + 100) star.y = -100;

        if (warpSpeedEnabled) {
            const speed = Math.sqrt(star.vx ** 2 + star.vy ** 2);
            const trailLength = speed * 15;

            ctx.beginPath();
            ctx.moveTo(star.x - star.vx * trailLength, star.y - star.vy * trailLength);
            ctx.lineTo(star.x, star.y);

            const gradient = ctx.createLinearGradient(
                star.x - star.vx * trailLength,
                star.y - star.vy * trailLength,
                star.x,
                star.y
            );

            if (colorShiftEnabled && star.hue !== undefined) {
                gradient.addColorStop(0, `hsla(${star.hue}, 80%, 70%, 0)`);
                gradient.addColorStop(1, `hsla(${star.hue}, 80%, 70%, ${star.opacity})`);
            } else {
                gradient.addColorStop(0, 'rgba(255, 255, 255, 0)');
                gradient.addColorStop(1, `rgba(255, 255, 255, ${star.opacity})`);
            }

            ctx.strokeStyle = gradient;
            ctx.lineWidth = star.radius * 2;
            ctx.stroke();
        }

        ctx.beginPath();
        const renderDepth = star.depth ** 1.2;
        const depthSize = star.radius * (0.3 + renderDepth * 0.7);
        const depthOpacity = star.opacity * (0.2 + renderDepth * 0.8);

        ctx.arc(star.x, star.y, depthSize, 0, Math.PI * 2);

        if (colorShiftEnabled && star.hue !== undefined) {
            const atmosphericHue = star.hue + (1 - star.depth) * 20;
            const atmosphericSaturation = 80 - (1 - star.depth) * 30;
            ctx.fillStyle = `hsla(${atmosphericHue}, ${atmosphericSaturation}%, 70%, ${depthOpacity})`;
        } else {
            const blueShift = Math.floor((1 - star.depth) * 30);
            ctx.fillStyle = `rgba(${255 - blueShift}, ${255 - blueShift}, 255, ${depthOpacity})`;
        }
        ctx.fill();

        if (star.depth > 0.7 && depthOpacity > 0.5) {
            ctx.beginPath();
            ctx.arc(star.x, star.y, depthSize * 1.5, 0, Math.PI * 2);
            const glowOpacity = (star.depth - 0.7) * 0.15;
            if (colorShiftEnabled && star.hue !== undefined) {
                ctx.fillStyle = `hsla(${star.hue}, 80%, 70%, ${glowOpacity})`;
            } else {
                ctx.fillStyle = `rgba(255, 255, 255, ${glowOpacity})`;
            }
            ctx.fill();
        }
    });

    for (let i = particles.length - 1; i >= 0; i--) {
        const particle = particles[i];
        particle.x += particle.vx;
        particle.y += particle.vy;
        particle.life--;

        if (particle.life <= 0) {
            particles.splice(i, 1);
            continue;
        }

        const lifeRatio = particle.life / particle.maxLife;
        ctx.beginPath();
        ctx.arc(particle.x, particle.y, particle.radius * lifeRatio, 0, Math.PI * 2);
        ctx.fillStyle = `hsla(${particle.hue}, 80%, 70%, ${lifeRatio * 0.8})`;
        ctx.fill();
    }

    requestAnimationFrame(animate);
}

window.addEventListener('resize', () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    width = canvas.width;
    height = canvas.height;
    bigBangCenter.x = width / 2;
    bigBangCenter.y = height / 2;
}); window.addEventListener('mousemove', (e) => {
    mouse.x = e.clientX;
    mouse.y = e.clientY;
});

canvas.addEventListener('click', (e) => {
    const particleCount = 30;
    for (let i = 0; i < particleCount; i++) {
        const angle = (Math.PI * 2 * i) / particleCount;
        const speed = Math.random() * 3 + 2;
        particles.push({
            x: e.clientX,
            y: e.clientY,
            vx: Math.cos(angle) * speed,
            vy: Math.sin(angle) * speed,
            radius: Math.random() * 2 + 1,
            life: 60,
            maxLife: 60,
            hue: Math.random() * 360
        });
    }
});

window.addEventListener('keydown', (e) => {
    const key = e.key.toLowerCase();

    if (key === 'g') {
        gravityEnabled = !gravityEnabled;
    } else if (key === 'c') {
        connectionsEnabled = !connectionsEnabled;
    } else if (key === 'a') {
        attractionMode = !attractionMode;
        if (attractionMode) repulsionMode = false;
    } else if (key === 'r') {
        repulsionMode = !repulsionMode;
        if (repulsionMode) attractionMode = false;
    } else if (key === 't') {
        trailsEnabled = !trailsEnabled;
    } else if (key === 'w') {
        warpSpeedEnabled = !warpSpeedEnabled;
    } else if (key === 'x') {
        colorShiftEnabled = !colorShiftEnabled;
    } else if (key === 'b') {
        blackHoleEnabled = !blackHoleEnabled;
        if (blackHoleEnabled) {
            blackHoleMass = 5000;
            eventHorizonRadius = 30;
        }
    } else if (key === 'm') {
        mouseParallaxEnabled = !mouseParallaxEnabled;
    }

    updateControlUI();
});

document.querySelectorAll('.control-key').forEach(element => {
    element.addEventListener('click', () => {
        const key = element.getAttribute('data-key');
        if (key) {
            const event = new KeyboardEvent('keydown', { key });
            window.dispatchEvent(event);
        }
    });
});

updateControlUI();
animate();

const observerOptions = {
    threshold: 0.5,
    rootMargin: '0px'
};

const sectionObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('active');

            const sectionId = entry.target.id;
            const newSection = sections.indexOf(sectionId);

            if (newSection !== currentSection && !isTransitioning) {
                transitionStars(currentSection, newSection);
                currentSection = newSection;
            }
        }
    });
}, observerOptions);

document.querySelectorAll('.section').forEach(section => {
    sectionObserver.observe(section);
});

function transitionStars(_fromSection: number, toSection: number) {
    isTransitioning = true;

    const sectionCenterY = toSection * height + height / 2;
    stars.forEach((star) => {
        const distanceToSection = Math.abs(star.y - (sectionCenterY - targetScrollY));
        if (distanceToSection < 400) {
            const transitionDepth = star.depth ** 1.5;
            const rippleStrength = (1 - distanceToSection / 400) * 1.2 * transitionDepth;
            const angle = Math.atan2(star.y - (sectionCenterY - targetScrollY), star.x - width / 2);

            star.vx += Math.cos(angle) * rippleStrength;
            star.vy += Math.sin(angle) * rippleStrength;
        }
    });

    setTimeout(() => {
        isTransitioning = false;
    }, 1000);
} const scrollIndicator = document.getElementById('scroll-indicator');
window.addEventListener('scroll', () => {
    if (scrollIndicator && window.scrollY > 100) {
        scrollIndicator.style.opacity = '0';
    } else if (scrollIndicator) {
        scrollIndicator.style.opacity = '1';
    }
});