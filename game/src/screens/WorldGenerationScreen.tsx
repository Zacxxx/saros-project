import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
    worldId: string;
    onComplete: () => void;
}

interface Progress {
    percent: number;
    status: string;
    done: boolean;
}

export default function WorldGenerationScreen({ worldId, onComplete }: Props) {
    const [progress, setProgress] = useState<Progress>({ percent: 0, status: "Starting...", done: false });
    const [started, setStarted] = useState(false);

    useEffect(() => {
        if (started) return;
        setStarted(true);
        invoke("generate_world", { id: worldId }).catch(console.error);
    }, [worldId, started]);

    useEffect(() => {
        const interval = setInterval(async () => {
            try {
                const p = await invoke<Progress>("get_generation_progress");
                setProgress(p);
                if (p.done) {
                    clearInterval(interval);
                    setTimeout(onComplete, 600);
                }
            } catch (e) {
                console.error(e);
            }
        }, 200);
        return () => clearInterval(interval);
    }, [onComplete]);

    return (
        <div style={styles.root} data-interactive>
            <div style={styles.bg} />
            <div style={styles.content}>
                <h1 style={styles.title}>SAROS</h1>
                <p style={styles.subtitle}>Generating World</p>

                <div style={styles.barOuter}>
                    <div
                        style={{
                            ...styles.barInner,
                            width: `${Math.min(progress.percent, 100)}%`,
                        }}
                    />
                </div>

                <div style={styles.percentRow}>
                    <span style={styles.statusText}>{progress.status}</span>
                    <span style={styles.percentText}>{Math.floor(progress.percent)}%</span>
                </div>

                <div style={styles.dots}>
                    {[0, 1, 2].map(i => (
                        <span key={i} style={{
                            ...styles.dot,
                            animationDelay: `${i * 0.3}s`,
                        }} />
                    ))}
                </div>
            </div>

            <style>{`
        @keyframes pulse-dot {
          0%, 80%, 100% { opacity: 0.3; transform: scale(0.8); }
          40% { opacity: 1; transform: scale(1.2); }
        }
        @keyframes gradient-shift {
          0% { background-position: 0% 50%; }
          50% { background-position: 100% 50%; }
          100% { background-position: 0% 50%; }
        }
      `}</style>
        </div>
    );
}

const styles: Record<string, React.CSSProperties> = {
    root: {
        position: "absolute",
        inset: 0,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        zIndex: 1000,
        pointerEvents: "all",
    },
    bg: {
        position: "absolute",
        inset: 0,
        background: "linear-gradient(135deg, #0a0a1a 0%, #1a0a2e 25%, #0a1a2e 50%, #0a0a1a 75%, #1a0a2e 100%)",
        backgroundSize: "400% 400%",
        animation: "gradient-shift 8s ease infinite",
    },
    content: {
        position: "relative",
        zIndex: 1,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: "16px",
        width: "420px",
    },
    title: {
        fontSize: "48px",
        fontWeight: "bold",
        letterSpacing: "12px",
        color: "#f0c040",
        textShadow: "0 0 30px rgba(240, 192, 64, 0.4)",
        margin: 0,
    },
    subtitle: {
        fontSize: "14px",
        color: "#8888aa",
        letterSpacing: "3px",
        textTransform: "uppercase" as const,
        margin: 0,
    },
    barOuter: {
        width: "100%",
        height: "6px",
        borderRadius: "3px",
        background: "rgba(255,255,255,0.08)",
        overflow: "hidden",
        marginTop: "12px",
    },
    barInner: {
        height: "100%",
        borderRadius: "3px",
        background: "linear-gradient(90deg, #f0c040, #ff8020)",
        transition: "width 0.3s ease-out",
        boxShadow: "0 0 12px rgba(240, 192, 64, 0.5)",
    },
    percentRow: {
        display: "flex",
        justifyContent: "space-between",
        width: "100%",
        fontSize: "12px",
    },
    statusText: {
        color: "#8888aa",
    },
    percentText: {
        color: "#f0c040",
        fontWeight: "bold",
        fontVariantNumeric: "tabular-nums",
    },
    dots: {
        display: "flex",
        gap: "8px",
        marginTop: "20px",
    },
    dot: {
        width: "8px",
        height: "8px",
        borderRadius: "50%",
        background: "#f0c040",
        animation: "pulse-dot 1.4s ease-in-out infinite",
    },
};
