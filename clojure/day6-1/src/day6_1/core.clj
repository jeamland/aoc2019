(ns day6-1.core
  (:gen-class))

(defn source-lines [] (line-seq (java.io.BufferedReader. *in*)))

(defn orbit-map [source]
  (into {} (map (fn [s] (vec (reverse (clojure.string/split s #"\)")))) source)))

(defn orbit-list [body orbits s]
  (if (= body "COM")
    s
    (recur (orbits body) orbits (concat s [body]))))

(defn -main
  [& _args]
  (let [orbits (orbit-map (source-lines))]
    (println
     (reduce +
             (map count
                  (map (fn [b] (orbit-list b orbits [])) (keys orbits)))))))
