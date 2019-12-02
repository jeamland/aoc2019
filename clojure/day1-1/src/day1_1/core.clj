(ns day1-1.core
  (:gen-class))

(defn fuel_for_mass [mass] (- (int (Math/floor (/ mass 3))) 2))

(defn -main
  [& _args]
  (println "Total:" (apply + (map fuel_for_mass (map #(Integer/parseInt %) (line-seq (java.io.BufferedReader. *in*)))))))
